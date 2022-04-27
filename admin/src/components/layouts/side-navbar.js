import React, { useEffect, useState } from "react";
import { Layout, Menu, Tooltip } from "antd";
import { Link } from "react-router-dom";
import axios from "axios";
import {
  SwitcherOutlined,
  ShareAltOutlined,
  WifiOutlined,
  UngroupOutlined,
  CloudServerOutlined,
  SettingOutlined,
} from "@ant-design/icons";
import { BiLogOutCircle } from "react-icons/bi";
import { FiEdit } from "react-icons/fi";
import Avatar1 from "../../assets/images/avatar/avatar.png";

const { Sider } = Layout;
const { SubMenu } = Menu;

const rootSubmenuKeys = ["/network", "/management"];

const baseUrl = process.env.REACT_APP_API_URL;
const getToken = localStorage.getItem("token");

const SideNavBar = () => {
  const [openKeys, setOpenKeys] = useState(["/status"]);
  const [currentUser, setCurrentUser] = useState();
  const [, setLoading] = useState(false);

  // -------------get user ----------

  useEffect(() => {
    setLoading(true);
    const auth = {
      Authorization: "Bearer " + getToken,
    };
    axios({
      method: "GET",
      url: `${baseUrl}/user/query`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setCurrentUser(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

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
        width="400px"
        breakpoint="lg"
        collapsedWidth="0"
      >
        <div className="logos">
          <Link to="/status">
            <img
              src="/images/icons/Koompi-white.png"
              className="logo-content-server"
              alt="logo"
            />

            <span>CONTENT SERVER</span>
          </Link>
          <div className="logout-icon">
            <Link to="/logout">
              <Tooltip placement="right" title="Logout">
                <BiLogOutCircle className="logo-logout" />
              </Tooltip>
            </Link>
          </div>
        </div>
        <div className="side-profile">
          <center>
            <img className="avatarNavbar" src={Avatar1} alt="avatar" />
            <div className="icon-user-setting">
              <Link to="/management/users-account">
                <Tooltip placement="right" title="User Setting">
                  <FiEdit className="account-icon" />
                </Tooltip>
              </Link>
            </div>
            <div className="popover-text">{currentUser}</div>
          </center>
        </div>
        <Menu
          defaultSelectedKeys="/status"
          openKeys={openKeys}
          onOpenChange={onOpenChange}
          theme="light"
          mode="inline"
        >
          <Menu.Item
            key="/status"
            icon={<SwitcherOutlined className="menu-icons" />}
          >
            <Link to="/status">STATUS</Link>
          </Menu.Item>
          <SubMenu key="/network" icon={<ShareAltOutlined />} title="NETWORK">
            <Menu.Item key="/network/wan">
              <Link to="/network/wan">WAN</Link>
            </Menu.Item>
            <Menu.Item key="/network/wlan">
              <Link to="/network/wlan">WLAN</Link>
            </Menu.Item>
          </SubMenu>
          <Menu.Item key="/wireless" icon={<WifiOutlined />}>
            <Link to="/wireless">WIRELESS</Link>
          </Menu.Item>
          <Menu.Item key="/dns" icon={<UngroupOutlined />}>
            <Link to="/dns">DNS</Link>
          </Menu.Item>
          <Menu.Item key="/storages" icon={<CloudServerOutlined />}>
            <Link to="/storages">STOGRAGES</Link>
          </Menu.Item>

          <SubMenu
            key="/management"
            icon={<SettingOutlined />}
            title="SETTINGS"
          >
            <Menu.Item key="/management/system-update">
              <Link to="/management/system-update">SYSTEM UPDATE</Link>
            </Menu.Item>
            <Menu.Item key="/management/users-account">
              <Link to="/management/users-account">USER ACCOUNT</Link>
            </Menu.Item>
            <Menu.Item key="/management/backup-restore">
              <Link to="/management/backup-restore">BACKUP & RESTORE</Link>
            </Menu.Item>
            <Menu.Item key="/management/reset">
              <Link to="/management/reset">RESET ALL</Link>
            </Menu.Item>
            <Menu.Item key="/management/time-setting">
              <Link to="/management/time-setting">TIME SETTING</Link>
            </Menu.Item>
          </SubMenu>
        </Menu>
      </Sider>
    </React.Fragment>
  );
};

export default SideNavBar;
