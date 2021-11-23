fn get_config_schema() {
    let config_schema = Builder::build(|params| {
        params.req_nested("description", Builder::string());
    });
}
