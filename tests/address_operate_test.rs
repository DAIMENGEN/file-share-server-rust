use file_share_server::common_operate;

#[test]
fn test_get_address() {
    let address = common_operate::get_address();

    assert_eq!(common_operate::Address::new("10.150.115.97".to_string(),"9090".to_string(), "10.150.115.97".to_string()), address);

}