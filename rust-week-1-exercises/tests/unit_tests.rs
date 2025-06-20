use rust_week_1_exercises::extract_tx_version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tx_version() {
        let tx_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
        let version = extract_tx_version(tx_hex).unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn test_version_2() {
        let tx_hex = "02000000000101706dc474338179f4ab8b7f0a4d07a2050113d7a0a9d21162e98b7319b102d3050100000000fdffffff02c9e10100000000001600148744bf9d300850a598b1a891f9a8d66524a4773065fc000000000000160014d1fae9a4de635c9c2e576238251d71be28a34dff0247304402201bf91432bbb345dcaa883a14fb7f18df7c821b160cc693f242112ba1a0acbdeb0220541b082c5fd4174f8eae782e213c1ebfc87b0598740ee0ef8463474debe83817012102062aea304064469ed250f46622e411de7eff4f07703e4273df6c80d58954ac2f00000000";
        let version = extract_tx_version(tx_hex).unwrap();
        assert_eq!(version, 2);
    }

    #[test]
    fn test_short_input() {
        let tx_hex = "00";
        let err = extract_tx_version(tx_hex).unwrap_err();
        assert!(err.contains("Transaction data too short"));
    }

    #[test]
    fn test_invalid_hex() {
        let tx_hex = "zzzzzzzz";
        let err = extract_tx_version(tx_hex).unwrap_err();
        assert!(err.contains("Hex decode error"));
    }
}
