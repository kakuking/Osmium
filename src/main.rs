use osmium::application::app::OsmiumEngine;

fn main() {
    let app = OsmiumEngine::new();
    
    unsafe {
        app.run();
    }
}
