import React, { useState } from "react";
import { Layout, Menu } from "antd";
import { Link } from "react-router-dom";
import {
  SwitcherOutlined,
  ShareAltOutlined,
  WifiOutlined,
  UngroupOutlined,
  CloudServerOutlined,
  SettingOutlined,
  ReadOutlined,
} from "@ant-design/icons";

const { Sider } = Layout;
const { SubMenu } = Menu;

const rootSubmenuKeys = ["/network", "/management"];

const SideNavBar = () => {
  const [openKeys, setOpenKeys] = useState(["/status"]);

  const onOpenChange = (keys) => {
    const latestOpenKey = keys.find((key) => openKeys.indexOf(key) === -1);
    if (rootSubmenuKeys.indexOf(latestOpenKey) === -1) {
      setOpenKeys(keys);
    } else {
      setOpenKeys(latestOpenKey ? [latestOpenKey] : []);
    }
  };

  return (
    <React.Fragment>
      <Sider
        style={{
          boxShadow: " 18px 0px 35px 0px rgba(0, 0, 0, 0.02)",
        }}
        theme="light"
        width="280px"
        breakpoint="lg"
        collapsedWidth="0"
      >
        <Menu
          openKeys={openKeys}
          theme="light"
          mode="inline"
          onOpenChange={onOpenChange}
        >
          <Menu.Item
            key="/status"
            icon={<SwitcherOutlined />}
            className="menu-navbar"
          >
            <Link to="/status">Status</Link>
          </Menu.Item>

          <SubMenu
            key="/network"
            icon={<ShareAltOutlined />}
            title="Network"
            className="menu-navbar"
          >
            <Menu.Item key="/network/wan" className="container-submenu">
              <Link to="/network/wan">WAN</Link>
            </Menu.Item>
            <Menu.Item key="/network/wlan" className="container-submenu">
              <Link to="/network/wlan">WLAN</Link>
            </Menu.Item>
          </SubMenu>

          <Menu.Item
            key="/wireless"
            icon={<WifiOutlined />}
            className="menu-navbar"
          >
            <Link to="/wireless">Wireless</Link>
          </Menu.Item>

          <Menu.Item
            key="/dns"
            icon={<UngroupOutlined />}
            className="menu-navbar"
          >
            <Link to="/dns">DNS</Link>
          </Menu.Item>

          <Menu.Item
            key="/storages"
            icon={<CloudServerOutlined />}
            className="menu-navbar"
          >
            <Link to="/storages">Storages</Link>
          </Menu.Item>

          <SubMenu
            key="/management"
            icon={<SettingOutlined />}
            title="Management"
            className="menu-navbar"
          >
            {/* <Menu.Item
              key="/management/system-update"
              className="container-submenu"
            >
              <Link to="/management/system-update">System Updates</Link>
            </Menu.Item> */}
            <Menu.Item
              key="/management/users-account"
              className="container-submenu"
            >
              <Link to="/management/users-account">Users Account</Link>
            </Menu.Item>
            <Menu.Item
              key="/management/backup-restore"
              className="container-submenu"
            >
              <Link to="/management/backup-restore">Backup & Restore</Link>
            </Menu.Item>
            <Menu.Item key="/management/reset" className="container-submenu">
              <Link to="/management/reset">Reset All</Link>
            </Menu.Item>
            <Menu.Item
              key="/management/time-setting"
              className="container-submenu"
            >
              <Link to="/management/time-setting">Time Settings</Link>
            </Menu.Item>
          </SubMenu>
          <Menu.Item
            key="/about-us"
            icon={<ReadOutlined />}
            className="menu-navbar"
          >
            <Link to="/about-us">About us</Link>
          </Menu.Item>
        </Menu>

        {/* <Menu defaultSelectedKeys="/status" theme="light" mode="inline">
        

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
          </SubMenu> */}

        {/* <SubMenu key="/website" icon={<GlobalOutlined />} title="Website">
            <Menu.Item key="/website/custom">
              <Link to="/website/custom">Custom Hosting</Link>
            </Menu.Item>
            <Menu.Item key="/website/internal">
              <Link to="/website/internal">Internal Website</Link>
            </Menu.Item>
            <Menu.Item key="/website/download">
              <Link to="/website/download">Download Website</Link>
            </Menu.Item>
          </SubMenu>          */}
        {/* </Menu> */}
      </Sider>
    </React.Fragment>
  );
};

export default SideNavBar;
