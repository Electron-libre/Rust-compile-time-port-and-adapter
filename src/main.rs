use env_logger::Builder;
use log::{debug, LevelFilter};

// With no adapter flag we use the DefaultAdapter
// else the adapter is not selected
#[cfg(not(feature = "other_adapter"))]
use DefaultAdapter as PortAdapter;

// With the other_adapter specified as feature flag we enable the OtherAdapter
#[cfg(feature = "other_adapter")]
use OtherAdapter as PortAdapter;

// Declare the Port as a trait to be implemented by adapters
trait Port {
    fn do_something(&mut self) -> bool;
}

// Implement the DefaultAdapter
#[cfg(not(feature = "other_adapter"))]
struct DefaultAdapter(bool);

#[cfg(not(feature = "other_adapter"))]
impl Port for DefaultAdapter {
    fn do_something(&mut self) -> bool {
        self.0 = true;
        debug!("Default Adapter done something");
        self.0
    }
}

// Implement the Other Adapter
#[cfg(any(feature = "other_adapter", test))]
struct OtherAdapter(bool);

#[cfg(any(feature = "other_adapter", test))]
impl Port for OtherAdapter {
    fn do_something(&mut self) -> bool {
        self.0 = true;
        debug!("Other Adapter done something");
        self.0
    }
}

// This illustrate a piece of code that depend on the Port.
fn consume_the_port<P: Port>(p: &mut P) -> bool {
    p.do_something()
}

fn main() {
    let mut logger = Builder::new();
    logger.filter(None, LevelFilter::Debug).init();

    let mut port_adapter = PortAdapter(false);
    consume_the_port(&mut port_adapter);
}

#[cfg(test)]
mod test {
    use super::*;

    // Use the TestAdapter as default implementation for this test suite.
    use TestAdapter as PortAdapter;

    // Define the test adapter and implement Port trait
    struct TestAdapter(bool);

    impl Port for TestAdapter {
        fn do_something(&mut self) -> bool {
            self.0 = true;
            debug!("Test Adapter done something");
            self.0
        }
    }

    // We want to display logs beside tests results for the sake of the demonstration
    fn init_logger() {
        let mut logger = Builder::new();
        let _ = logger
            .filter(None, LevelFilter::Debug)
            .is_test(true)
            .try_init();
    }

    // Define a test for the Port behaviour
    fn test_port_adapter_behaviour<P: Port>(p: &mut P) {
        init_logger();
        let consumed = consume_the_port(p);
        assert!(consumed)
    }

    // Then test against each implementations
    #[test]
    fn test_adapter_do_something() {
        let mut port_adapter = PortAdapter(false);
        test_port_adapter_behaviour(&mut port_adapter)
    }

    #[test]
    fn default_adapter_do_something() {
        use DefaultAdapter as PortAdapter;

        let mut port_adapter = PortAdapter(false);
        test_port_adapter_behaviour(&mut port_adapter)
    }

    #[test]
    fn other_adapter_do_something() {
        use OtherAdapter as PortAdapter;

        let mut port_adapter = PortAdapter(false);
        test_port_adapter_behaviour(&mut port_adapter)
    }
}
