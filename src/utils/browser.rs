use crate::utils::launcher::look_path;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

const URL: &str = "https://lbs.qq.com/getPoint/";
const SEARCH_INPUT_SELECT: &str = "#app > div > div > div.layout-view > div > div.getpoint-map > div.getpoint-search > div > div > div > div > input";
const SEARCH_CLEAR_SELECT: &str = "#app > div > div > div.layout-view > div > div.getpoint-map > div.getpoint-search > div > div > div > div > div > div.getpoint-search-clear";

const NAME_SELECT: &str =
    "#app > div > div > div.layout-view > div > div.getpoint-info > div.getpoint-info-content > h2";
const LOCATION_SELECT: &str = "#location > div > input";
const ADDRESS_SELECT: &str = "#address > div > input";

type BrowserResult<T> = Result<T, Box<dyn Error>>;

pub enum TencentMapRead {
    Name,
    Location,
    Address,
}

pub struct TencentMap {
    browser: Option<Arc<Browser>>,
}

impl TencentMap {
    pub fn new() -> BrowserResult<Self> {
        Ok(Self {
            browser: None,
            // debug,
        })
    }

    fn get_browser(&mut self) -> BrowserResult<Arc<Browser>> {
        if let Some(browser) = &self.browser {
            Ok(browser.clone())
        } else {
            let launch_options = LaunchOptions {
                headless: false,
                enable_gpu: true,
                // enable_logging: true,
                path: look_path(),
                // port: Some(9222),
                idle_browser_timeout: Duration::from_secs(86400),
                ..Default::default()
            };
            let browser = Browser::new(launch_options)?;
            let browser = Arc::new(browser);
            self.browser = Some(browser.clone());

            Ok(browser.clone())
        }
    }

    fn set_browser_none(&mut self) {
        // self.debug_println("set_browser_none");
        self.browser = None;
    }

    // fn debug_println(&self, error: impl std::fmt::Display) {
    //     if self.debug {
    //         println!("{}", error);
    //     }
    // }

    fn get_tab(&mut self) -> BrowserResult<Arc<Tab>> {
        let browser = self.get_browser()?;

        if let Ok(tabs) = browser.get_tabs().lock() {
            for tab in tabs.iter() {
                if tab.get_url().contains(URL) {
                    return Ok(tab.clone());
                }
            }
        } else {
            // self.debug_println("获取标签列表失败");
            return Err("获取标签列表失败".into());
        }

        let tab = match browser.new_tab() {
            Ok(tab) => tab,
            Err(_) => {
                self.set_browser_none();
                // self.debug_println(e);
                self.get_tab()?
            }
        };
        tab.set_default_timeout(Duration::from_secs(20))
            .navigate_to(URL)
            .map_err(|_| {
                self.set_browser_none();
                // self.debug_println(e);
                "打开腾讯地图失败"
            })?
            .wait_until_navigated()
            .map_err(|_| {
                self.set_browser_none();
                // self.debug_println(e);
                "等待腾讯地图加载失败"
            })?;

        Ok(tab)
    }

    pub fn search(&mut self, query: &str) -> BrowserResult<()> {
        let tab = self.get_tab()?;

        if let Ok(ele) = tab.find_element(SEARCH_CLEAR_SELECT) {
            ele.click()?;
        }
        tab.wait_for_element(SEARCH_INPUT_SELECT)
            .map_err(|_| {
                // self.debug_println(e);
                "等待搜索框失败"
            })?
            .click()
            .map_err(|_| {
                // self.debug_println(e);
                "点击搜索框失败"
            })?;

        // tab.wait_for_element(SEARCH_INPUT_SELECT)?.click()?;
        tab.send_character(query)
            .map_err(|_| {
                // self.debug_println(e);
                "输入搜索内容失败"
            })?
            .press_key("Enter")
            .map_err(|_| {
                // self.debug_println(e);
                "搜索失败"
            })?;

        Ok(())
    }

    pub fn read(&mut self, read: TencentMapRead) -> BrowserResult<Option<String>> {
        let tab = self.get_tab()?;

        let select = match read {
            TencentMapRead::Name => {
                return if let Ok(name) = tab
                    .wait_for_element(NAME_SELECT)
                    .map_err(|_| {
                        // self.debug_println(e);
                        "获取名称失败"
                    })?
                    .get_inner_text()
                {
                    if name == "点图获取坐标" {
                        Ok(None)
                    } else {
                        Ok(Some(name))
                    }
                } else {
                    Ok(None)
                };
            }
            TencentMapRead::Location => LOCATION_SELECT,
            TencentMapRead::Address => ADDRESS_SELECT,
        };

        if let Some(value) = tab
            .wait_for_element(select)
            .map_err(|_| {
                // self.debug_println(e);
                match read {
                    TencentMapRead::Location => "获取坐标失败",
                    TencentMapRead::Address => "获取地址失败",
                    _ => "获取失败",
                }
            })?
            .get_attribute_value("value")
            .map_err(|_| {
                // self.debug_println(e);
                match read {
                    TencentMapRead::Location => "获取坐标失败",
                    TencentMapRead::Address => "获取地址失败",
                    _ => "获取失败",
                }
            })?
        {
            if value == "-" {
                Ok(None)
            } else {
                Ok(Some(value))
            }
        } else {
            Ok(None)
        }
    }
}
