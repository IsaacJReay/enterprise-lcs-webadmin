import React from "react";
import { Layout, Avatar, Popover, Row, Col } from "antd";
import { Link } from "react-router-dom";
import { HiLogout } from "react-icons/hi";
import { FiSettings } from "react-icons/fi";
import Avatar1 from "../../assets/images/avatar/avatar.png";

const { Header } = Layout;

const NavBar = () => {
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
            |CONTENT SERVER
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
                    <div className="popover-text">Thith THIN</div>
                    <span>thiththin762@gmail.com</span>
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
                <Link to="/ ">
                  <Col
                    className="logout"
                    style={{ paddingTop: "4px", color: "black" }}
                    span={20}
                  >
                    <p>Setting</p>
                  </Col>
                </Link>
              </Row>
              <Row className="accountNavbarhover">
                <Col style={{ paddingTop: "6px" }} span={4}>
                  <HiLogout style={{ fontSize: "20px", color: "black" }} />
                </Col>
                <Link to="/logout">
                  <Col
                    className="logout"
                    style={{ paddingTop: "4px", color: "red" }}
                    span={20}
                  >
                    <p>Logout</p>
                  </Col>
                </Link>
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
