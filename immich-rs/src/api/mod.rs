// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! Generated API client endpoints for Immich.

pub mod activities;
pub mod albums;
pub mod api_keys;
pub mod assets;
pub mod authentication;
pub mod common;
pub mod database;
pub mod duplicates;
pub mod jobs;
pub mod libraries;
pub mod maintenance;
pub mod map;
pub mod memories;
pub mod notifications;
pub mod partners;
pub mod people;
pub mod plugins;
pub mod queues;
pub mod search;
pub mod server;
pub mod sessions;
pub mod shared_links;
pub mod sync;
pub mod system;
pub mod tags;
pub mod timeline;
pub mod trash;
pub mod users;
pub mod workflows;

impl crate::ImmichClient {
    /// Returns an API handle for the [`activities`](crate::api::activities::Activities) endpoints.
    #[must_use]
    pub fn activities(&self) -> activities::Activities<'_> {
        activities::Activities::new(self)
    }
    /// Returns an API handle for the [`albums`](crate::api::albums::Albums) endpoints.
    #[must_use]
    pub fn albums(&self) -> albums::Albums<'_> {
        albums::Albums::new(self)
    }
    /// Returns an API handle for the [`api_keys`](crate::api::api_keys::ApiKeys) endpoints.
    #[must_use]
    pub fn api_keys(&self) -> api_keys::ApiKeys<'_> {
        api_keys::ApiKeys::new(self)
    }
    /// Returns an API handle for the [`assets`](crate::api::assets::Assets) endpoints.
    #[must_use]
    pub fn assets(&self) -> assets::Assets<'_> {
        assets::Assets::new(self)
    }
    /// Returns an API handle for the [`download`](crate::api::assets::download::Download) endpoints.
    #[must_use]
    pub fn assets_download(&self) -> assets::download::Download<'_> {
        assets::download::Download::new(self)
    }
    /// Returns an API handle for the [`faces`](crate::api::assets::faces::Faces) endpoints.
    #[must_use]
    pub fn assets_faces(&self) -> assets::faces::Faces<'_> {
        assets::faces::Faces::new(self)
    }
    /// Returns an API handle for the [`stacks`](crate::api::assets::stacks::Stacks) endpoints.
    #[must_use]
    pub fn assets_stacks(&self) -> assets::stacks::Stacks<'_> {
        assets::stacks::Stacks::new(self)
    }
    /// Returns an API handle for the [`authentication`](crate::api::authentication::Authentication) endpoints.
    #[must_use]
    pub fn authentication(&self) -> authentication::Authentication<'_> {
        authentication::Authentication::new(self)
    }
    /// Returns an API handle for the [`common`](crate::api::common::Common) endpoints.
    #[must_use]
    pub fn common(&self) -> common::Common<'_> {
        common::Common::new(self)
    }
    /// Returns an API handle for the [`admin`](crate::api::database::admin::Admin) endpoints.
    #[must_use]
    pub fn database_admin(&self) -> database::admin::Admin<'_> {
        database::admin::Admin::new(self)
    }
    /// Returns an API handle for the [`duplicates`](crate::api::duplicates::Duplicates) endpoints.
    #[must_use]
    pub fn duplicates(&self) -> duplicates::Duplicates<'_> {
        duplicates::Duplicates::new(self)
    }
    /// Returns an API handle for the [`jobs`](crate::api::jobs::Jobs) endpoints.
    #[must_use]
    pub fn jobs(&self) -> jobs::Jobs<'_> {
        jobs::Jobs::new(self)
    }
    /// Returns an API handle for the [`libraries`](crate::api::libraries::Libraries) endpoints.
    #[must_use]
    pub fn libraries(&self) -> libraries::Libraries<'_> {
        libraries::Libraries::new(self)
    }
    /// Returns an API handle for the [`admin`](crate::api::maintenance::admin::Admin) endpoints.
    #[must_use]
    pub fn maintenance_admin(&self) -> maintenance::admin::Admin<'_> {
        maintenance::admin::Admin::new(self)
    }
    /// Returns an API handle for the [`map`](crate::api::map::Map) endpoints.
    #[must_use]
    pub fn map(&self) -> map::Map<'_> {
        map::Map::new(self)
    }
    /// Returns an API handle for the [`memories`](crate::api::memories::Memories) endpoints.
    #[must_use]
    pub fn memories(&self) -> memories::Memories<'_> {
        memories::Memories::new(self)
    }
    /// Returns an API handle for the [`notifications`](crate::api::notifications::Notifications) endpoints.
    #[must_use]
    pub fn notifications(&self) -> notifications::Notifications<'_> {
        notifications::Notifications::new(self)
    }
    /// Returns an API handle for the [`admin`](crate::api::notifications::admin::Admin) endpoints.
    #[must_use]
    pub fn notifications_admin(&self) -> notifications::admin::Admin<'_> {
        notifications::admin::Admin::new(self)
    }
    /// Returns an API handle for the [`partners`](crate::api::partners::Partners) endpoints.
    #[must_use]
    pub fn partners(&self) -> partners::Partners<'_> {
        partners::Partners::new(self)
    }
    /// Returns an API handle for the [`people`](crate::api::people::People) endpoints.
    #[must_use]
    pub fn people(&self) -> people::People<'_> {
        people::People::new(self)
    }
    /// Returns an API handle for the [`plugins`](crate::api::plugins::Plugins) endpoints.
    #[must_use]
    pub fn plugins(&self) -> plugins::Plugins<'_> {
        plugins::Plugins::new(self)
    }
    /// Returns an API handle for the [`queues`](crate::api::queues::Queues) endpoints.
    #[must_use]
    pub fn queues(&self) -> queues::Queues<'_> {
        queues::Queues::new(self)
    }
    /// Returns an API handle for the [`search`](crate::api::search::Search) endpoints.
    #[must_use]
    pub fn search(&self) -> search::Search<'_> {
        search::Search::new(self)
    }
    /// Returns an API handle for the [`server`](crate::api::server::Server) endpoints.
    #[must_use]
    pub fn server(&self) -> server::Server<'_> {
        server::Server::new(self)
    }
    /// Returns an API handle for the [`sessions`](crate::api::sessions::Sessions) endpoints.
    #[must_use]
    pub fn sessions(&self) -> sessions::Sessions<'_> {
        sessions::Sessions::new(self)
    }
    /// Returns an API handle for the [`shared_links`](crate::api::shared_links::SharedLinks) endpoints.
    #[must_use]
    pub fn shared_links(&self) -> shared_links::SharedLinks<'_> {
        shared_links::SharedLinks::new(self)
    }
    /// Returns an API handle for the [`sync`](crate::api::sync::Sync) endpoints.
    #[must_use]
    pub fn sync(&self) -> sync::Sync<'_> {
        sync::Sync::new(self)
    }
    /// Returns an API handle for the [`config`](crate::api::system::config::Config) endpoints.
    #[must_use]
    pub fn system_config(&self) -> system::config::Config<'_> {
        system::config::Config::new(self)
    }
    /// Returns an API handle for the [`metadata`](crate::api::system::metadata::Metadata) endpoints.
    #[must_use]
    pub fn system_metadata(&self) -> system::metadata::Metadata<'_> {
        system::metadata::Metadata::new(self)
    }
    /// Returns an API handle for the [`tags`](crate::api::tags::Tags) endpoints.
    #[must_use]
    pub fn tags(&self) -> tags::Tags<'_> {
        tags::Tags::new(self)
    }
    /// Returns an API handle for the [`timeline`](crate::api::timeline::Timeline) endpoints.
    #[must_use]
    pub fn timeline(&self) -> timeline::Timeline<'_> {
        timeline::Timeline::new(self)
    }
    /// Returns an API handle for the [`trash`](crate::api::trash::Trash) endpoints.
    #[must_use]
    pub fn trash(&self) -> trash::Trash<'_> {
        trash::Trash::new(self)
    }
    /// Returns an API handle for the [`users`](crate::api::users::Users) endpoints.
    #[must_use]
    pub fn users(&self) -> users::Users<'_> {
        users::Users::new(self)
    }
    /// Returns an API handle for the [`admin`](crate::api::users::admin::Admin) endpoints.
    #[must_use]
    pub fn users_admin(&self) -> users::admin::Admin<'_> {
        users::admin::Admin::new(self)
    }
    /// Returns an API handle for the [`workflows`](crate::api::workflows::Workflows) endpoints.
    #[must_use]
    pub fn workflows(&self) -> workflows::Workflows<'_> {
        workflows::Workflows::new(self)
    }
}
