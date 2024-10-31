use crate::route::route::Route;
pub struct Service{
    pub router: Vec<Route>,
}

//new
impl Service{
    pub fn new() -> Service{
        Service{
            router: vec![],
        }
    }

    // get router
    pub fn get_router(&self) -> Vec<Route>{
        self.router.clone()
    }

    // set router
    pub fn set_router(&mut self, router: Vec<Route>){
        self.router = router;
    }
}

// clone
impl Clone for Service{
    fn clone(&self) -> Self{
        Service{
            router: self.router.clone(),
        }
    }
}
