use local_doc_assistant::ContextManager;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
        let root = if cwd.ends_with("local-doc-assistant") {
        cwd.parent().map(|p| p.to_path_buf()).unwrap_or(cwd.clone())
    } else { cwd };
    let cm = ContextManager::new(root);
    cm.ensure_templates_exist()?;
    let txt = cm.load_socket_api_context()?;
    println!("Loaded template length: {}", txt.len());
    Ok(())
}

