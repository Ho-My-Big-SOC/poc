use elasticsearch::{Elasticsearch, http::transport::Transport};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Création du transport
    let transport = Transport::single_node("http://localhost:9200")?;
    let client = Elasticsearch::new(transport);

    // Document JSON à insérer
    let document = json!({
        "title": "Titre de test",
        "content": "Contenu de test pour Elasticsearch"
    });

    // Insertion du document dans l'index "mon_index"
    let response = client
        .index(elasticsearch::IndexParts::Index("mon_index"))
        .body(document)
        .send()
        .await?;

    // Vérification du succès de la requête
    if response.status_code().is_success() {
        println!("Document inséré avec succès.");
    } else {
        println!("Erreur lors de l'insertion du document.");
    }

    Ok(())
}
