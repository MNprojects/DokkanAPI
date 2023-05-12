#[cfg(test)]
mod tests {
    use crate::server::index;
    use crate::types::structs::AppState;
    use actix_web::{ http::{ header::ContentType }, test, App, web };

    use once_cell::sync::Lazy;

    static APP_STATE: Lazy<AppState> = Lazy::new(|| {
        // Initialize your AppState here, e.g., by reading the long file
        AppState::new()
    });
    #[test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_filter_by_fname() {
        // test code for filter by async fname
    }

    #[actix_web::test]
    async fn test_filter_by_title() {
        // test code for filter by title
    }

    #[actix_web::test]
    async fn test_filter_by_ftitle() {
        // test code for filter by ftitle
    }

    #[actix_web::test]
    async fn test_filter_by_has_transformation() {
        // test code for filter by has_trasformation
    }

    #[actix_web::test]
    async fn test_filter_by_num() {
        // test code for filter by num
    }

    #[actix_web::test]
    async fn test_filter_by_cost() {
        // test code for filter by cost
    }

    #[actix_web::test]
    async fn test_filter_by_id() {
        // test code for filter by id
    }

    #[actix_web::test]
    async fn test_filter_by_type() {
        // test code for filter by type
    }

    #[actix_web::test]
    async fn test_filter_by_rarity() {
        // test code for filter by rarity
    }

    #[actix_web::test]
    async fn test_filter_by_links() {
        // test code for filter by links
    }

    #[actix_web::test]
    async fn test_filter_by_categories() {
        // test code for filter by categories
    }

    #[actix_web::test]
    async fn test_filter_by_class() {
        // test code for filter by class
    }

    #[actix_web::test]
    async fn test_sort_by_max_level_attack() {
        // test code for sort by max level attack
    }

    #[actix_web::test]
    async fn test_sort_by_reverse_max_level_attack() {
        // test code for sort by reverse max level attack
    }
}