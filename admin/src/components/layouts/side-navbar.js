import React, { useState, useEffect } from "react";
import { Layout, Menu } from "antd";
import { Link } from "react-router-dom";
import {
  SwitcherOutlined,
  ShareAltOutlined,
  WifiOutlined,
  UngroupOutlined,
  CloudServerOutlined,
  SettingOutlined,
  GlobalOutlined,
} from "@ant-design/icons";

const { Sider } = Layout;
const { SubMenu } = Menu;

const SideNavBar = () => {
  const [pathname, setPathname] = useState(window.location.pathname);

  // const subMenu = window.location.pathname.split("/")[2];

  useEffect(() => {
    setPathname(window.location.pathname);
  }, [pathname]);

  // const rootSubmenuKeys = ["network", "storage", "management", "website"];

  // const [openKeys, setOpenKeys] = useState([pathname]);

  // const onOpenChange = (keys) => {
  //   const latestOpenKey = keys.find((key) => openKeys.indexOf(key) === -1);
  //   if (rootSubmenuKeys.indexOf(latestOpenKey) === -1) {
  //     setOpenKeys(keys);
  //   } else {
  //     setOpenKeys(latestOpenKey ? [latestOpenKey] : []);
  //   }
  // };

  return (
    <React.Fragment>
      <Sider
        style={{
          boxShadow: " 18px 0px 35px 0px rgba(0, 0, 0, 0.02)",
        }}
        theme="light"
        width="350px"
        breakpoint="lg"
        collapsedWidth="0"
      >
        <Menu defaultSelectedKeys="/status" theme="light" mode="inline">
          {/* <Row gutter={[24, 12]}>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/status">
                <Menu.Item key="/dashboard/status" icon={<SwitcherOutlined />}>
                  <Link to="/dashboard/status">Status</Link>
                </Menu.Item>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/network">
                <SubMenu
                  key="/dashboard/network"
                  icon={<ShareAltOutlined />}
                  title="Network"
                >
                  <div className="container-submenu">
                    <Menu.Item key="/dashboard/network/wan">
                      <Link to="/dashboard/network/wan">WAN</Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/network/wlan">
                      <Link to="/dashboard/network/wlan">WLAN</Link>
                    </Menu.Item>
                  </div>
                </SubMenu>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/wireless">
                <Menu.Item key="/dashboard/wireless" icon={<WifiOutlined />}>
                  <Link to="/dashboard/wireless">Wireless</Link>
                </Menu.Item>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/dhcp">
                <Menu.Item key="/dashboard/dhcp" icon={<HddOutlined />}>
                  <Link to="/dashboard/dhcp">DHCP</Link>
                </Menu.Item>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/dns">
                <Menu.Item key="/dashboard/dns" icon={<UngroupOutlined />}>
                  <Link to="/dashboard/dns">DNS</Link>
                </Menu.Item>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/storage">
                <SubMenu
                  key="/dashboard/storage"
                  icon={<CloudServerOutlined />}
                  title="Storage"
                >
                  <div className="container-submenu">
                    <Menu.Item key="/dashboard/storage/sumsung">
                      <Link to="/dashboard/storage/sumsung">Samsung EVO</Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/storage/USB">
                      <Link to="/dashboard/storage/USB">USB</Link>
                    </Menu.Item>
                  </div>
                </SubMenu>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/management">
                <SubMenu
                  key="/dashboard/management"
                  icon={<SettingOutlined />}
                  title="Management"
                >
                  <div className="container-submenu">
                    <Menu.Item key="/dashboard/management/system-update">
                      <Link to="/dashboard/management/system-update">
                        System Updates
                      </Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/management/users-account">
                      <Link to="/dashboard/management/users-account">
                        Users Account
                      </Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/management/export">
                      <Link to="/dashboard/management/export">Export</Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/management/import">
                      <Link to="/dashboard/management/import">Import</Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/management/reset">
                      <Link to="/dashboard/management/reset">Reset All</Link>
                    </Menu.Item>
                  </div>
                </SubMenu>
              </div>
            </Col>
            <Col span={24}>
              <div className="menu-navbar" key="/dashboard/website">
                <SubMenu
                  key="/dashboard/website"
                  icon={<GlobalOutlined />}
                  title="Website"
                >
                  <div className="container-submenu">
                    <Menu.Item key="/dashboard/website/View">
                      <Link to="/dashboard/website/View">View</Link>
                    </Menu.Item>
                    <Menu.Item key="/dashboard/website/hosting">
                      <Link to="/dashboard/website/hosting">Web Hosting</Link>
                    </Menu.Item>
                  </div>
                </SubMenu>
              </div>
            </Col>
          </Row> */}

          <Menu.Item key="/status" icon={<SwitcherOutlined />}>
            <Link to="/status">Status</Link>
          </Menu.Item>
          <SubMenu key="/network" icon={<ShareAltOutlined />} title="Network">
            <Menu.Item key="/network/wan">
              <Link to="/network/wan">WAN</Link>
            </Menu.Item>
            <Menu.Item key="/network/wlan">
              <Link to="/network/wlan">WLAN</Link>
            </Menu.Item>
          </SubMenu>
          <Menu.Item key="/wireless" icon={<WifiOutlined />}>
            <Link to="/wireless">Wireless</Link>
          </Menu.Item>
          <Menu.Item key="/dns" icon={<UngroupOutlined />}>
            <Link to="/dns">DNS</Link>
          </Menu.Item>
          <Menu.Item key="/storages" icon={<CloudServerOutlined />}>
            <Link to="/storages">Storages</Link>
          </Menu.Item>

          <SubMenu
            key="/management"
            icon={<SettingOutlined />}
            title="Management"
          >
            <Menu.Item key="/management/system-update">
              <Link to="/management/system-update">System Updates</Link>
            </Menu.Item>
            <Menu.Item key="/management/users-account">
              <Link to="/management/users-account">Users Account</Link>
            </Menu.Item>
            <Menu.Item key="/management/backup-restore">
              <Link to="/management/backup-restore">Backup & Restore</Link>
            </Menu.Item>
            <Menu.Item key="/management/reset">
              <Link to="/management/reset">Reset All</Link>
            </Menu.Item>
            <Menu.Item key="/management/time-setting">
              <Link to="/management/time-setting">Time Settings</Link>
            </Menu.Item>
          </SubMenu>

          <SubMenu key="/website" icon={<GlobalOutlined />} title="Website">
            <Menu.Item key="/website/internal">
              <Link to="/website/internal">Internal Website</Link>
            </Menu.Item>
            <Menu.Item key="/website/custom">
              <Link to="/website/custom">Custom Hosting</Link>
            </Menu.Item>
            <Menu.Item key="/website/download">
              <Link to="/website/download">Download Website</Link>
            </Menu.Item>
          </SubMenu>

          <Link to="/about-us">
            <a className="about-us">About US</a>
          </Link>
        </Menu>
      </Sider>
    </React.Fragment>
  );
};

export default SideNavBar;
