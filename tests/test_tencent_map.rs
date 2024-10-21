use headless_chrome_operation_map::utils::browser::{TencentMap, TencentMapRead};
use serial_test::serial;
#[test]
#[serial(frpc)]
fn test_search() -> Result<(), String> {
    let mut tm = TencentMap::default();

    tm.search("三里屯".to_string().as_str())
        .map_err(|e| format!("search error: {:?}", e))?;

    Ok(())
}

#[test]
#[serial(frpc)]
fn test_read() -> Result<(), String> {
    let mut tm = TencentMap::default();

    let name = tm
        .read(TencentMapRead::Name)
        .map_err(|e| format!("read name error: {:?}", e))?;
    assert_eq!(name, None);

    let location = tm
        .read(TencentMapRead::Location)
        .map_err(|e| format!("read location error: {:?}", e))?;
    assert_eq!(location, None);

    let address = tm
        .read(TencentMapRead::Address)
        .map_err(|e| format!("read address error: {:?}", e))?;
    assert_eq!(address, None);

    tm.search("三里屯".to_string().as_str())
        .map_err(|e| format!("search error: {:?}", e))?;

    let name = tm
        .read(TencentMapRead::Name)
        .map_err(|e| format!("read name error: {:?}", e))?;
    assert_eq!(name, Some("三里屯".to_string()));

    let location = tm
        .read(TencentMapRead::Location)
        .map_err(|e| format!("read location error: {:?}", e))?;
    assert_eq!(location, Some("39.937657,116.453508".to_string()));

    let address = tm
        .read(TencentMapRead::Address)
        .map_err(|e| format!("read address error: {:?}", e))?;
    assert_eq!(address, Some("北京市朝阳区".to_string()));
    Ok(())
}
