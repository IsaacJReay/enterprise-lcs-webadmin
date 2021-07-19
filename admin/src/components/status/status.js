import React, { useEffect, useState } from "react";
import { Layout, Col, Row } from "antd";
import axios from "axios";

const { Content } = Layout;

const getToken = localStorage.getItem("token");

const StatusPage = () => {
  const [, setLoading] = useState(false);
  const [items, setItems] = useState([]);

  useEffect(() => {
    setLoading(true);
    const auth = {
      Authorization: "Bearer " + getToken,
    };
    axios({
      method: "GET",
      url: "http://10.42.0.188:8002/private/api/settings/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setItems(res.data);

        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  return (
    <React.Fragment>
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
                    {items && (
                      <React.Fragment>
                        <Col>
                          <div className="desc-details-left">
                            <p>MAC address : </p>
                            <p>IP Address : </p>
                            <p>Subnet Mask : </p>
                            <p>Default Getway : </p>
                          </div>
                        </Col>
                        <Col>
                          <div className="desc-details-right">
                            <p>{items.wan_macaddress} </p>
                            <p>{items.wan_ip} </p>
                            <p>{items.wan_netmask}</p>
                            <p>{items.wan_gateway}</p>
                          </div>
                        </Col>
                      </React.Fragment>
                    )}
                  </Row>
                </div>
              </div>
              <hr />
              <div className="container-details">
                <h2>WLAN</h2>
                <div className="desc-container">
                  <Row gutter={[64, 0]}>
                    {items && (
                      <React.Fragment>
                        <Col>
                          <div className="desc-details-left">
                            <p>MAC Address : </p>
                            <p>IP Address : </p>
                            <p>Subnet Mask : </p>
                            <p>DNS : </p>
                            <p>SSID : </p>
                            <p>HW mode : </p>
                            <p>Channel : </p>
                            <p>HW n mode : </p>
                            <p>QOS : </p>
                          </div>
                        </Col>
                        <Col>
                          <div className="desc-details-right">
                            <p>{items.wlan_macaddress} </p>
                            <p>{items.wlan_ip} </p>
                            <p>{items.wlan_netmask} </p>
                            <p>{items.wlan_dns} </p>
                            <p>{items.wlan_ssid} </p>
                            <p>{items.wlan_hw_mode} </p>
                            <p>{items.wlan_channel} </p>
                            <p>{items.wlan_hw_n_mode ? "true" : "false"} </p>
                            <p>{items.wlan_qos ? "true" : "false"} </p>
                          </div>
                        </Col>
                      </React.Fragment>
                    )}
                  </Row>
                </div>
              </div>
              {/* <hr />
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
              </div> */}
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
    </React.Fragment>
  );
};

export default StatusPage;
