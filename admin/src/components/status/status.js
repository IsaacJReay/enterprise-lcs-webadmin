import React from "react";
import { Layout, Col, Row } from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";

const { Content } = Layout;

const StatusPage = () => {
  return (
    <React.Fragment>
      <Layout style={{ minHeight: "100vh" }}>
        <NavBar />
        <Layout>
          <SideNavBar />
          <Content>
            <Row gutter={[32, 32]}>
              <Col span={16}>
                <div className="container">
                  <div className="container-header">
                    <h1>STATUS</h1>
                  </div>
                  <hr />
                  <div className="desc-container-banner">
                    <Row gutter={[64, 0]}>
                      <Col>
                        <div className="desc-details-left">
                          <p>Fireware Version : </p>
                          <p>Hardware Version : </p>
                        </div>
                      </Col>
                      <Col>
                        <div className="desc-details-right">
                          <p>32.2.3 Build 2021 </p>
                          <p>kmpv2021.1 </p>
                        </div>
                      </Col>
                    </Row>
                  </div>
                  <hr />
                  <div className="container-details">
                    <h2>WAN</h2>
                    <div className="desc-container">
                      <Row gutter={[64, 0]}>
                        <Col>
                          <div className="desc-details-left">
                            <p>MAC Address : </p>
                            <p>IP Address : </p>
                            <p>Subnet Mask : </p>
                            <p>Default Getway : </p>
                          </div>
                        </Col>
                        <Col>
                          <div className="desc-details-right">
                            <p>F8-F8-F8-F8-F8-F8-F8 </p>
                            <p>192.168.0.10 </p>
                            <p>255.255.255.0.10 </p>
                            <p>192.168.0.1 </p>
                          </div>
                        </Col>
                      </Row>
                    </div>
                  </div>
                  <hr />
                  <div className="container-details">
                    <h2>WLAN</h2>
                    <div className="desc-container">
                      <Row gutter={[64, 0]}>
                        <Col>
                          <div className="desc-details-left">
                            <p>MAC Address : </p>
                            <p>IP Address : </p>
                            <p>Subnet Mask : </p>
                            <p>Default Getway : </p>
                          </div>
                        </Col>
                        <Col>
                          <div className="desc-details-right">
                            <p>F8-F8-F8-F8-F8-F8-F8 </p>
                            <p>192.168.0.10 </p>
                            <p>255.255.255.0.10 </p>
                            <p>192.168.0.1 </p>
                          </div>
                        </Col>
                      </Row>
                    </div>
                  </div>
                  <hr />
                  <div className="container-details">
                    <h2>Wireless</h2>
                    <div className="desc-container">
                      <Row gutter={[64, 0]}>
                        <Col>
                          <div className="desc-details-left">
                            <p>MAC Address : </p>
                            <p>IP Address : </p>
                            <p>Subnet Mask : </p>
                            <p>Default Getway : </p>
                          </div>
                        </Col>
                        <Col>
                          <div className="desc-details-right">
                            <p>F8-F8-F8-F8-F8-F8-F8 </p>
                            <p>192.168.0.10 </p>
                            <p>255.255.255.0.10 </p>
                            <p>192.168.0.1 </p>
                          </div>
                        </Col>
                      </Row>
                    </div>
                  </div>
                </div>
              </Col>
              <Col span={8}>
                <div className="container">
                  <div className="container-header">
                    <h1>Desciptions</h1>
                  </div>
                </div>
              </Col>
            </Row>
          </Content>
        </Layout>
      </Layout>
    </React.Fragment>
  );
};

export default StatusPage;
