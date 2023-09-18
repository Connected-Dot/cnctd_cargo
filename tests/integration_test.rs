#[cfg(test)]
mod tests {
    use cnctd_cargo::Cargo;

    #[tokio::test]
    async fn test_commands() {
        Cargo::check_for_rust_and_cargo().await.unwrap()
    }
}
