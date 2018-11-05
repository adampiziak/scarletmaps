use juniper::Context;
use model::{Database, Route};

impl Context for Database {}

graphql_object!(Route: Database | &self | {
    field id() -> &String {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field active() -> &bool {
        &self.active
    }
});


graphql_object!(Database: Database as "Query" |&self| {
    description: "The root query object of the schema"

    field routes(&executor, active: Option<bool>) -> Vec<&Route> {
        match active {
            Some(a) => executor.context().get_all().into_iter().filter(|r| r.is_active() == a).collect(),
            None => executor.context().get_all()
        }
    }

    field route(
        &executor,
        id: String as "id of the route"
    ) -> Option<&Route> {
        executor.context().get_route(&id)
    }
});

