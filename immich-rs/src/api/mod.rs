// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! Generated API client endpoints for Immich.

pub mod activities;
pub mod admin;
pub mod albums;
pub mod api_keys;
pub mod assets;
pub mod authentication;
pub mod clusters;
pub mod duplicates;
pub mod jobs;
pub mod libraries;
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
pub mod views;
pub mod workflows;

impl crate::ImmichClient {
    /// Returns an API handle for the [`activities`](crate::api::activities::Activities) endpoints.
    #[inline]
    #[must_use]
    pub const fn activities(&self) -> activities::Activities<'_> {
        activities::Activities::new(self)
    }

    /// Returns an API handle for the [`admin`](crate::api::admin::Admin) endpoints.
    #[inline]
    #[must_use]
    pub const fn admin(&self) -> admin::Admin<'_> {
        admin::Admin::new(self)
    }

    /// Returns an API handle for the [`albums`](crate::api::albums::Albums) endpoints.
    #[inline]
    #[must_use]
    pub const fn albums(&self) -> albums::Albums<'_> {
        albums::Albums::new(self)
    }

    /// Returns an API handle for the [`api_keys`](crate::api::api_keys::ApiKeys) endpoints.
    #[inline]
    #[must_use]
    pub const fn api_keys(&self) -> api_keys::ApiKeys<'_> {
        api_keys::ApiKeys::new(self)
    }

    /// Returns an API handle for the [`assets`](crate::api::assets::Assets) endpoints.
    #[inline]
    #[must_use]
    pub const fn assets(&self) -> assets::Assets<'_> {
        assets::Assets::new(self)
    }

    /// Returns an API handle for the [`authentication`](crate::api::authentication::Authentication) endpoints.
    #[inline]
    #[must_use]
    pub const fn authentication(&self) -> authentication::Authentication<'_> {
        authentication::Authentication::new(self)
    }

    /// Returns an API handle for the [`clusters`](crate::api::clusters::Clusters) endpoints.
    #[inline]
    #[must_use]
    pub const fn clusters(&self) -> clusters::Clusters<'_> {
        clusters::Clusters::new(self)
    }

    /// Returns an API handle for the [`duplicates`](crate::api::duplicates::Duplicates) endpoints.
    #[inline]
    #[must_use]
    pub const fn duplicates(&self) -> duplicates::Duplicates<'_> {
        duplicates::Duplicates::new(self)
    }

    /// Returns an API handle for the [`jobs`](crate::api::jobs::Jobs) endpoints.
    #[inline]
    #[must_use]
    pub const fn jobs(&self) -> jobs::Jobs<'_> {
        jobs::Jobs::new(self)
    }

    /// Returns an API handle for the [`libraries`](crate::api::libraries::Libraries) endpoints.
    #[inline]
    #[must_use]
    pub const fn libraries(&self) -> libraries::Libraries<'_> {
        libraries::Libraries::new(self)
    }

    /// Returns an API handle for the [`map`](crate::api::map::Map) endpoints.
    #[inline]
    #[must_use]
    pub const fn map(&self) -> map::Map<'_> {
        map::Map::new(self)
    }

    /// Returns an API handle for the [`memories`](crate::api::memories::Memories) endpoints.
    #[inline]
    #[must_use]
    pub const fn memories(&self) -> memories::Memories<'_> {
        memories::Memories::new(self)
    }

    /// Returns an API handle for the [`notifications`](crate::api::notifications::Notifications) endpoints.
    #[inline]
    #[must_use]
    pub const fn notifications(&self) -> notifications::Notifications<'_> {
        notifications::Notifications::new(self)
    }

    /// Returns an API handle for the [`partners`](crate::api::partners::Partners) endpoints.
    #[inline]
    #[must_use]
    pub const fn partners(&self) -> partners::Partners<'_> {
        partners::Partners::new(self)
    }

    /// Returns an API handle for the [`people`](crate::api::people::People) endpoints.
    #[inline]
    #[must_use]
    pub const fn people(&self) -> people::People<'_> {
        people::People::new(self)
    }

    /// Returns an API handle for the [`plugins`](crate::api::plugins::Plugins) endpoints.
    #[inline]
    #[must_use]
    pub const fn plugins(&self) -> plugins::Plugins<'_> {
        plugins::Plugins::new(self)
    }

    /// Returns an API handle for the [`queues`](crate::api::queues::Queues) endpoints.
    #[inline]
    #[must_use]
    pub const fn queues(&self) -> queues::Queues<'_> {
        queues::Queues::new(self)
    }

    /// Returns an API handle for the [`search`](crate::api::search::Search) endpoints.
    #[inline]
    #[must_use]
    pub const fn search(&self) -> search::Search<'_> {
        search::Search::new(self)
    }

    /// Returns an API handle for the [`server`](crate::api::server::Server) endpoints.
    #[inline]
    #[must_use]
    pub const fn server(&self) -> server::Server<'_> {
        server::Server::new(self)
    }

    /// Returns an API handle for the [`sessions`](crate::api::sessions::Sessions) endpoints.
    #[inline]
    #[must_use]
    pub const fn sessions(&self) -> sessions::Sessions<'_> {
        sessions::Sessions::new(self)
    }

    /// Returns an API handle for the [`shared_links`](crate::api::shared_links::SharedLinks) endpoints.
    #[inline]
    #[must_use]
    pub const fn shared_links(&self) -> shared_links::SharedLinks<'_> {
        shared_links::SharedLinks::new(self)
    }

    /// Returns an API handle for the [`sync`](crate::api::sync::Sync) endpoints.
    #[inline]
    #[must_use]
    pub const fn sync(&self) -> sync::Sync<'_> {
        sync::Sync::new(self)
    }

    /// Returns an API handle for the [`system`](crate::api::system::System) endpoints.
    #[inline]
    #[must_use]
    pub const fn system(&self) -> system::System<'_> {
        system::System::new(self)
    }

    /// Returns an API handle for the [`tags`](crate::api::tags::Tags) endpoints.
    #[inline]
    #[must_use]
    pub const fn tags(&self) -> tags::Tags<'_> {
        tags::Tags::new(self)
    }

    /// Returns an API handle for the [`timeline`](crate::api::timeline::Timeline) endpoints.
    #[inline]
    #[must_use]
    pub const fn timeline(&self) -> timeline::Timeline<'_> {
        timeline::Timeline::new(self)
    }

    /// Returns an API handle for the [`trash`](crate::api::trash::Trash) endpoints.
    #[inline]
    #[must_use]
    pub const fn trash(&self) -> trash::Trash<'_> {
        trash::Trash::new(self)
    }

    /// Returns an API handle for the [`users`](crate::api::users::Users) endpoints.
    #[inline]
    #[must_use]
    pub const fn users(&self) -> users::Users<'_> {
        users::Users::new(self)
    }

    /// Returns an API handle for the [`views`](crate::api::views::Views) endpoints.
    #[inline]
    #[must_use]
    pub const fn views(&self) -> views::Views<'_> {
        views::Views::new(self)
    }

    /// Returns an API handle for the [`workflows`](crate::api::workflows::Workflows) endpoints.
    #[inline]
    #[must_use]
    pub const fn workflows(&self) -> workflows::Workflows<'_> {
        workflows::Workflows::new(self)
    }
}
