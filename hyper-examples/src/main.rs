use std::collections::{HashMap, HashSet};
use bytes::Bytes;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use route_recognizer::Params;
use router::Router;
use std::sync::Arc;

mod handler;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State(usize);

#[derive(Debug)]
struct NFA {
    states: HashSet<State>,
    transitions: HashMap<(State, Option<char>), HashSet<State>>,
    start_state: State,
    accept_states: HashSet<State>,
}

impl NFA {
    fn new(start_state: State, accept_states: HashSet<State>) -> Self {
        NFA {
            states: HashSet::new(),
            transitions: HashMap::new(),
            start_state,
            accept_states,
        }
    }

    fn add_state(&mut self, state: State) {
        self.states.insert(state);
    }

    fn add_transition(&mut self, from: State, input: Option<char>, to: State) {
        self.transitions
            .entry((from, input))
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    fn is_accepting(&self, input: &str) -> bool {
        let mut current_states = HashSet::new();
        current_states.insert(self.start_state.clone());

        for c in input.chars() {
            let mut next_states = HashSet::new();
            for state in &current_states {
                if let Some(states) = self.transitions.get(&(state.clone(), Some(c))) {
                    next_states.extend(states.clone());
                }
                if let Some(states) = self.transitions.get(&(state.clone(), None)) {
                    next_states.extend(states.clone());
                }
            }
            current_states = next_states;
        }

        current_states
            .iter()
            .any(|state| self.accept_states.contains(state))
    }
}

#[tokio::main]
async fn main() {
    let some_state = "state".to_string();

    let mut router: Router = Router::new();
    router.get("/test", Box::new(handler::test_handler));
    router.post("/send", Box::new(handler::send_handler));
    router.get("/params/:some_param", Box::new(handler::param_handler));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = "127.0.0.1:8080".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let _ = server.await;


    let start_state = State(0);
    let accept_states = vec![State(1)].into_iter().collect();

    let mut nfa = NFA::new(start_state.clone(), accept_states);
    nfa.add_state(start_state.clone());
    nfa.add_state(State(1));

    nfa.add_transition(start_state.clone(), Some('a'), State(0));
    nfa.add_transition(start_state.clone(), Some('a'), State(1));
    nfa.add_transition(State(1), Some('b'), State(1));

    let input = "aab";
    println!("Does the NFA accept '{}'? {}", input, nfa.is_accepting(input));

}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}
