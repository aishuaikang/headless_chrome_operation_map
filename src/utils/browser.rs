use crate::utils::launcher::look_path;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use super::regions::Region;

const URL: &str = "https://lbs.qq.com/getPoint/";
// 搜索框
const MAP_SEARCH_BAR_SELECT: &str = "#app > div > div > div.layout-view > div > div.getpoint-map > div.getpoint-search > div > div > div > div > input";
// 搜索框清除按钮
const MAP_SEARCH_BAR_CLEAR_BUTTON_SELECT: &str = "#app > div > div > div.layout-view > div > div.getpoint-map > div.getpoint-search > div > div > div > div > div > div.getpoint-search-clear";
// 热门城市
const MAP_AREA_POPULAR_CITIES_SELECT: &str = "#city-select > div > ul.hotcity-list > li > span";
// 分类城市
const MAP_AREA_CATEGORIES_SELECT: &str = "#categoresList > div > ul > ul > li > span";
// 当前地图区域
const MAP_AREA_SELECT: &str = "#city-select > p";

// 终端名称
const TERMINAL_NAME_SELECT: &str =
    "#app > div > div > div > div > div.getpoint-info > div.getpoint-info-content > h2";
// 终端经纬度
const TERMINAL_LOCATION_SELECT: &str = "#location > div > input";
// 终端地址
const TERMINAL_ADDRESS_SELECT: &str = "#address > div > input";

type BrowserResult<T> = Result<T, Box<dyn Error>>;

pub enum TencentMapRead {
    Name,
    Location,
    Address,
}

pub struct TencentMapOptions {
    pub tab_timeout: Option<Duration>,
    pub devtools: Option<bool>,
}

impl Default for TencentMapOptions {
    fn default() -> Self {
        Self {
            tab_timeout: None,
            devtools: None,
        }
    }
}

pub struct TencentMap {
    browser: Option<Arc<Browser>>,
    options: TencentMapOptions,
}

impl Default for TencentMap {
    fn default() -> Self {
        Self {
            browser: None,
            options: Default::default(),
        }
    }
}

impl TencentMap {
    pub fn new(options: TencentMapOptions) -> Self {
        Self {
            browser: None,
            options,
        }
    }

    fn get_browser(&mut self) -> BrowserResult<Arc<Browser>> {
        if let Some(browser) = &self.browser {
            Ok(browser.clone())
        } else {
            let launch_options = LaunchOptions {
                headless: false,
                enable_gpu: true,
                devtools: self.options.devtools.unwrap_or(false),
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
                self.get_tab()?
            }
        };
        tab.set_default_timeout(self.options.tab_timeout.unwrap_or(Duration::from_secs(60)))
            .navigate_to(URL)
            .map_err(|_| {
                self.set_browser_none();
                "打开腾讯地图失败"
            })?
            .wait_until_navigated()
            .map_err(|_| {
                self.set_browser_none();
                "等待腾讯地图加载失败"
            })?;

        Ok(tab)
    }

    pub fn search(&mut self, query: &str) -> BrowserResult<()> {
        let tab = self.get_tab()?;

        if let Ok(ele) = tab.find_element(MAP_SEARCH_BAR_CLEAR_BUTTON_SELECT) {
            ele.click()?;
        }
        tab.wait_for_element(MAP_SEARCH_BAR_SELECT)
            .map_err(|_| "等待搜索框失败")?
            .click()
            .map_err(|_| "点击搜索框失败")?;

        // tab.wait_for_element(SEARCH_INPUT_SELECT)?.click()?;
        tab.send_character(query)
            .map_err(|_| "输入搜索内容失败")?
            .press_key("Enter")
            .map_err(|_| "搜索失败")?;

        Ok(())
    }

    pub fn read(&mut self, read: TencentMapRead) -> BrowserResult<Option<String>> {
        let tab = self.get_tab()?;

        let select = match read {
            TencentMapRead::Name => {
                return if let Ok(name) = tab
                    .wait_for_element(TERMINAL_NAME_SELECT)
                    .map_err(|_| "获取名称失败")?
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
            TencentMapRead::Location => TERMINAL_LOCATION_SELECT,
            TencentMapRead::Address => TERMINAL_ADDRESS_SELECT,
        };

        if let Some(value) = tab
            .wait_for_element(select)
            .map_err(|_| match read {
                TencentMapRead::Location => "获取坐标失败",
                TencentMapRead::Address => "获取地址失败",
                _ => "获取失败",
            })?
            .get_attribute_value("value")
            .map_err(|_| match read {
                TencentMapRead::Location => "获取坐标失败",
                TencentMapRead::Address => "获取地址失败",
                _ => "获取失败",
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

    pub fn set_map_region_by_region_full_name(
        &mut self,
        region_full_name: &str,
    ) -> BrowserResult<()> {
        let region =
            Region::get_map_region_by_region_full_name(region_full_name).ok_or("获取地区失败")?;

        let tab = self.get_tab()?;

        tab.evaluate(
            &format!(
                r#"
                requestAnimationFrame(() => {{
                    const popular_cities_list = document.querySelectorAll('{}');
                    const categories_list = document.querySelectorAll('{}');

                    for (let i = 0; i < categories_list.length; i++) {{
                        const category = categories_list[i];
                        let name = "{}";
                        console.log(category.innerText , name);
                        if (category.innerText === name) {{
                            requestAnimationFrame(() => {{
                                category.click();
                            }});
                            break;
                        }}

                    }}

                    for (let i = 0; i < popular_cities_list.length; i++) {{
                        const popular_city = popular_cities_list[i];
                        let name = "{}";
                        console.log(popular_city.innerText , name);
                        if (popular_city.innerText === name) {{
                            requestAnimationFrame(() => {{
                                popular_city.click();
                            }});
                            break;
                        }}
                    }}
                }});
            "#,
                MAP_AREA_POPULAR_CITIES_SELECT,
                MAP_AREA_CATEGORIES_SELECT,
                region.name,
                region.name
            ),
            false,
        )?;

        Ok(())
    }

    pub fn get_map_region(&mut self) -> BrowserResult<String> {
        let tab = self.get_tab()?;

        Ok(tab
            .wait_for_element(MAP_AREA_SELECT)
            .map_err(|_| "等待当前地图区域失败")?
            .get_inner_text()
            .map_err(|_| "获取当前地图区域失败")?
            .trim()
            .to_string())
    }
}
