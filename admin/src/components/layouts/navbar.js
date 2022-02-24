import React, { useEffect, useState } from "react";
import { Layout, Avatar, Popover, Row, Col } from "antd";
import { Link } from "react-router-dom";
import { HiLogout } from "react-icons/hi";
import { FiSettings } from "react-icons/fi";
import { ReadOutlined } from "@ant-design/icons";
import axios from "axios";
import Avatar1 from "../../assets/images/avatar/avatar.png";

const { Header } = Layout;
const baseUrl = process.env.REACT_APP_API_URL;
const getToken = localStorage.getItem("token");

const NavBar = () => {
  const [currentUser, setCurrentUser] = useState({});
  const [, setLoading] = useState(false);

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

  return (
    <React.Fragment>
      <Header style={{ background: "#1B262C" }}>
        <div className="logos">
          <Link to="/status">
            <img
              src="/images/icons/Koompi-white.png"
              className="logo-content-server"
              alt="logo"
            />

            <span>CONTENT SERVER</span>
          </Link>
        </div>
        <Popover
          placement="bottomRight"
          title={
            <React.Fragment>
              <Row gutter={[32, 0]}>
                <Col span={4}>
                  <img className="avatarNavbar" src={Avatar1} alt="avatar" />
                </Col>
                <Col span={20}>
                  <div>
                    <div className="popover-text">{currentUser.username}</div>
                  </div>
                </Col>
              </Row>
            </React.Fragment>
          }
          content={
            <div style={{ width: "270px" }}>
              <Row className="accountNavbarhover">
                <Col style={{ paddingTop: "6px" }} span={4}>
                  <FiSettings style={{ fontSize: "20px", color: "black" }} />
                </Col>
                <Col className="logout" span={20}>
                  <Link to="/management/users-account">
                    <p style={{ paddingTop: "4px", color: "black" }}>Setting</p>
                  </Link>
                </Col>
              </Row>
              <Row className="accountNavbarhover">
                <Col span={4} style={{ paddingTop: "6px" }}>
                  <ReadOutlined className="about-us-icon" />
                </Col>
                <Col className="logout" span={20}>
                  <Link to="/about-us">
                    <a style={{ paddingTop: "4px", color: "black" }}>
                      About US
                    </a>
                  </Link>
                </Col>
              </Row>
              <Row className="accountNavbarhover">
                <Col style={{ paddingTop: "6px" }} span={4}>
                  <HiLogout style={{ fontSize: "20px", color: "black" }} />
                </Col>

                <Col className="logout" span={20}>
                  <Link to="/logout">
                    <p style={{ paddingTop: "4px", color: "red" }}>Logout</p>
                  </Link>
                </Col>
              </Row>
            </div>
          }
          trigger="click"
        >
          <div className="sub-topnavbar">
            <Avatar className="navbar-avatar" src={Avatar1} size={55} />
          </div>
        </Popover>
      </Header>
    </React.Fragment>
  );
};

export default NavBar;
