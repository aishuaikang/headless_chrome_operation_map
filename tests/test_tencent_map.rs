use std::{thread, time::Duration};

use headless_chrome_operation_map::utils::browser::{
    TencentMap, TencentMapOptions, TencentMapRead,
};
use serial_test::serial;
#[test]
#[serial(frpc)]
fn test_search() -> Result<(), String> {
    let mut tm = TencentMap::new(TencentMapOptions {
        tab_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    });

    tm.search("三里屯".to_string().as_str())
        .map_err(|e| format!("search error: {:?}", e))?;

    Ok(())
}

#[test]
#[serial(frpc)]
fn test_read() -> Result<(), String> {
    let mut tm = TencentMap::new(TencentMapOptions {
        tab_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    });

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

#[test]
#[serial(frpc)]
fn test_set_map_region_by_region_full_name() {
    let mut tm = TencentMap::new(TencentMapOptions {
        tab_timeout: Some(Duration::from_secs(60)),
        devtools: Some(true),
        ..Default::default()
    });

    let region_full_name = "贵州省#贵阳市#南明区";
    tm.set_map_region_by_region_full_name(region_full_name)
        .unwrap();

    let region_name = tm.get_map_region().unwrap();
    assert_eq!(region_name, "贵阳市");
}

#[test]
#[serial(frpc)]
fn test_get_map_region() {
    let mut tm = TencentMap::new(TencentMapOptions {
        tab_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    });

    let region_name = tm.get_map_region().unwrap();
    assert_eq!(region_name, "北京市");
}

#[test]
#[serial(frpc)]
fn test_exit() {
    let mut tm = TencentMap::new(TencentMapOptions {
        tab_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    });
    tm.get_map_region().unwrap();

    // 开始计时
    println!("10s后退出");
    thread::sleep(Duration::from_secs(10));
    tm.exit().unwrap();
    println!("exit");
    thread::sleep(Duration::from_secs(10));
    println!("exit end");
}
