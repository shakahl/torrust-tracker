pub type ReqwestQuery = Vec<ReqwestQueryParam>;
pub type ReqwestQueryParam = (String, String);

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub bind_address: String,
}

/// URL Query component
#[derive(Default, Debug)]
pub struct Query {
    params: Vec<QueryParam>,
}

impl Query {
    pub fn empty() -> Self {
        Self { params: vec![] }
    }

    pub fn params(params: Vec<QueryParam>) -> Self {
        Self { params }
    }

    pub fn add_param(&mut self, param: QueryParam) {
        self.params.push(param);
    }
}

impl From<Query> for ReqwestQuery {
    fn from(url_search_params: Query) -> Self {
        url_search_params
            .params
            .iter()
            .map(|param| ReqwestQueryParam::from((*param).clone()))
            .collect()
    }
}

/// URL query param
#[derive(Clone, Debug)]
pub struct QueryParam {
    name: String,
    value: String,
}

impl QueryParam {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<QueryParam> for ReqwestQueryParam {
    fn from(param: QueryParam) -> Self {
        (param.name, param.value)
    }
}
