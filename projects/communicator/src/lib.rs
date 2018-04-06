// Public API declaration with contents extracted into separate file
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use std::panic;
    use super::client;
    use super::network;

    #[test]
    fn test_client_connect() {
        let actual_client_connection: bool = client::connect();
        let expected_response: bool = true;
        assert_eq!(actual_client_connection, expected_response);
    }

    #[test]
    fn test_network_connect() {
        let actual_network_connection: bool = network::connect();
        let expected_response: bool = true;
        assert_eq!(actual_network_connection, expected_response);
    }

    #[test]
    fn test_network_server_connect() {
        let actual_network_server_connection: bool = network::server::connect();
        let expected_response: bool = true;
        assert_eq!(actual_network_server_connection, expected_response);
    }
}
