/*
 * JJS main API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::option::Option;
use std::{borrow::Borrow, rc::Rc};

use futures::Future;
use hyper;
use serde_json;

use super::{configuration, request as __internal_request, Error};

pub struct DefaultApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> DefaultApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient { configuration }
    }
}

pub trait DefaultApi {
    fn api_version(
        &self,
    ) -> Box<dyn Future<Output = Result<crate::models::ApiVersion, Error<serde_json::Value>>> + Unpin>;
    fn create_user(
        &self,
        user_create_params: crate::models::UserCreateParams,
    ) -> Box<dyn Future<Output = Result<crate::models::User, Error<serde_json::Value>>> + Unpin>;
    fn delete_run(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>> + Unpin>;
    fn get_contest(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<crate::models::Contest, Error<serde_json::Value>>> + Unpin>;
    fn get_contest_standings(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<serde_json::Value, Error<serde_json::Value>>> + Unpin>;
    fn get_run(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>;
    fn get_run_binary(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<String, Error<serde_json::Value>>> + Unpin>;
    fn get_run_live_status(
        &self,
        id: i32,
    ) -> Box<
        dyn Future<Output = Result<crate::models::RunLiveStatusUpdate, Error<serde_json::Value>>>
            + Unpin,
    >;
    fn get_run_protocol(
        &self,
        id: i32,
        compile_log: Option<bool>,
        test_data: Option<bool>,
        output: Option<bool>,
        answer: Option<bool>,
    ) -> Box<dyn Future<Output = Result<serde_json::Value, Error<serde_json::Value>>> + Unpin>;
    fn get_run_source(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<String, Error<serde_json::Value>>> + Unpin>;
    fn is_dev(&self) -> Box<dyn Future<Output = Result<bool, Error<serde_json::Value>>> + Unpin>;
    fn list_contest_problems(
        &self,
        name: &str,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Problem>, Error<serde_json::Value>>> + Unpin,
    >;
    fn list_contests(
        &self,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Contest>, Error<serde_json::Value>>> + Unpin,
    >;
    fn list_runs(
        &self,
    ) -> Box<dyn Future<Output = Result<Vec<crate::models::Run>, Error<serde_json::Value>>> + Unpin>;
    fn list_toolchains(
        &self,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Toolchain>, Error<serde_json::Value>>>
            + Unpin,
    >;
    fn log_in(
        &self,
        simple_auth_params: crate::models::SimpleAuthParams,
    ) -> Box<
        dyn Future<Output = Result<crate::models::SessionToken, Error<serde_json::Value>>> + Unpin,
    >;
    fn patch_run(
        &self,
        id: i32,
        run_patch: Option<crate::models::RunPatch>,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>;
    fn submit_run(
        &self,
        run_simple_submit_params: crate::models::RunSimpleSubmitParams,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>;
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> DefaultApi
    for DefaultApiClient<C>
{
    fn api_version(
        &self,
    ) -> Box<dyn Future<Output = Result<crate::models::ApiVersion, Error<serde_json::Value>>> + Unpin>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/system/api-version".to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn create_user(
        &self,
        user_create_params: crate::models::UserCreateParams,
    ) -> Box<dyn Future<Output = Result<crate::models::User, Error<serde_json::Value>>> + Unpin>
    {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/users".to_string());
        req = req.with_body_param(user_create_params);

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn delete_run(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>> + Unpin> {
        let mut req =
            __internal_request::Request::new(hyper::Method::DELETE, "/runs/{id}".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_contest(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<crate::models::Contest, Error<serde_json::Value>>> + Unpin>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/contests/{name}".to_string());
        req = req.with_path_param("name".to_string(), name.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_contest_standings(
        &self,
        name: &str,
    ) -> Box<dyn Future<Output = Result<serde_json::Value, Error<serde_json::Value>>> + Unpin> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/contests/{name}/standings".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_run(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/runs/{id}".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_run_binary(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<String, Error<serde_json::Value>>> + Unpin> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/runs/<id>/binary".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_run_live_status(
        &self,
        id: i32,
    ) -> Box<
        dyn Future<Output = Result<crate::models::RunLiveStatusUpdate, Error<serde_json::Value>>>
            + Unpin,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/runs/<id>/live".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_run_protocol(
        &self,
        id: i32,
        compile_log: Option<bool>,
        test_data: Option<bool>,
        output: Option<bool>,
        answer: Option<bool>,
    ) -> Box<dyn Future<Output = Result<serde_json::Value, Error<serde_json::Value>>> + Unpin> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/runs/<id>/protocol".to_string());
        if let Some(ref s) = compile_log {
            req = req.with_query_param("compile_log".to_string(), s.to_string());
        }
        if let Some(ref s) = test_data {
            req = req.with_query_param("test_data".to_string(), s.to_string());
        }
        if let Some(ref s) = output {
            req = req.with_query_param("output".to_string(), s.to_string());
        }
        if let Some(ref s) = answer {
            req = req.with_query_param("answer".to_string(), s.to_string());
        }
        req = req.with_path_param("id".to_string(), id.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn get_run_source(
        &self,
        id: i32,
    ) -> Box<dyn Future<Output = Result<String, Error<serde_json::Value>>> + Unpin> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/runs/<id>/source".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn is_dev(&self) -> Box<dyn Future<Output = Result<bool, Error<serde_json::Value>>> + Unpin> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/system/is-dev".to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn list_contest_problems(
        &self,
        name: &str,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Problem>, Error<serde_json::Value>>> + Unpin,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/contests/{name}/problems".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn list_contests(
        &self,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Contest>, Error<serde_json::Value>>> + Unpin,
    > {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/contests".to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn list_runs(
        &self,
    ) -> Box<dyn Future<Output = Result<Vec<crate::models::Run>, Error<serde_json::Value>>> + Unpin>
    {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/runs".to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn list_toolchains(
        &self,
    ) -> Box<
        dyn Future<Output = Result<Vec<crate::models::Toolchain>, Error<serde_json::Value>>>
            + Unpin,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/toolchains".to_string());

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn log_in(
        &self,
        simple_auth_params: crate::models::SimpleAuthParams,
    ) -> Box<
        dyn Future<Output = Result<crate::models::SessionToken, Error<serde_json::Value>>> + Unpin,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/auth/simple".to_string());
        req = req.with_body_param(simple_auth_params);

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn patch_run(
        &self,
        id: i32,
        run_patch: Option<crate::models::RunPatch>,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::PATCH, "/runs/{id}".to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(run_patch);

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }

    fn submit_run(
        &self,
        run_simple_submit_params: crate::models::RunSimpleSubmitParams,
    ) -> Box<dyn Future<Output = Result<crate::models::Run, Error<serde_json::Value>>> + Unpin>
    {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/runs".to_string());
        req = req.with_body_param(run_simple_submit_params);

        // TODO: do not box here
        Box::new(req.execute(self.configuration.borrow()))
    }
}
