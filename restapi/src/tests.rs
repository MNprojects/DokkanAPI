#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{ http::{ self, header::ContentType }, test };
    #[actix_web::test]
    async fn test_filter_by_name() {
        let req = test::TestRequest
            ::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
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