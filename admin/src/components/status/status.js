import React, { useEffect, useState } from "react";
import { Layout, Col, Row, Spin } from "antd";
import axios from "axios";

const { Content } = Layout;

const getToken = localStorage.getItem("token");

const StatusPage = () => {
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState([]);

  useEffect(() => {
    setLoading(true);
    const auth = {
      Authorization: "Bearer " + getToken,
    };
    axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setStatus(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  if (loading) {
    return (
      <div className="spin">
        <Spin />
      </div>
    );
  }

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
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
                      {status && (
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
                              <p>{status.wan_macaddress} </p>
                              <p>{status.wan_ip} </p>
                              <p>{status.wan_netmask}</p>
                              <p>{status.wan_gateway}</p>
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
                      {status && (
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
                              <p>{status.wlan_macaddress} </p>
                              <p>{status.wlan_ip} </p>
                              <p>{status.wlan_netmask} </p>
                              <p>{status.wlan_dns} </p>
                              <p>{status.wlan_ssid} </p>
                              <p>{status.wlan_hw_mode} </p>
                              <p>{status.wlan_channel} </p>
                              <p>{status.wlan_hw_n_mode ? "true" : "false"} </p>
                              <p>{status.wlan_qos ? "true" : "false"} </p>
                            </div>
                          </Col>
                        </React.Fragment>
                      )}
                    </Row>
                  </div>
                </div>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
                </div>
                <div>
                  <h2>Status Help</h2>
                  <p>
                    The Status page displays the Router's current status and
                    configuration. All information is read-only.{" "}
                  </p>
                  <p>
                    <strong>WAN</strong> The following parameters apply to the
                    WAN ports of the Router. You can configure them in the
                    Network WAN page.
                  </p>
                  <ul>
                    <li>
                      <strong>MAC Address</strong> - The physical address of the
                      WAN port, as seen from the Internet.
                    </li>
                    <li>
                      <strong>IP Address</strong> - The current WAN (Internet)
                      IP Address. This field will be blank or 0.0.0.0 if the IP
                      Address is assigned dynamically and there is no connection
                      to Internet.
                    </li>
                    <li>
                      <strong>Subnet Mask</strong> - The subnet mask associated
                      with the WAN IP Address.
                    </li>
                    <li>
                      <strong>Default Gateway</strong> - The Gateway currently
                      used by the Router is shown here. When you use Dynamic IP
                      as the connection Internet type, the Renew button will be
                      displayed here. Click the Renew button to obtain new IP
                      parameters dynamically from the ISP. And if you have got
                      an IP address Release button will be displayed here. Click
                      the Release button to release the IP address the Router
                      has obtained from the ISP.{" "}
                    </li>
                  </ul>
                  <p>
                    <strong>WLAN</strong> - These are the current settings or
                    information for Wireless. You can configure them in the
                    Wireless Wireless Settings page.{" "}
                  </p>
                  <ul>
                    <li>
                      <strong>MAC Address</strong> - The physical address of the
                      WAN port, as seen from the Internet.
                    </li>
                    <li>
                      <strong>IP Address</strong> - The current WAN (Internet)
                      IP Address. This field will be blank or 0.0.0.0 if the IP
                      Address is assigned dynamically and there is no connection
                      to Internet.
                    </li>
                    <li>
                      <strong>Subnet Mask</strong> - The subnet mask associated
                      with the WAN IP Address.
                    </li>
                    <li>
                      <strong>DNS</strong> - The DNS (Domain Name System) Server
                      IP addresses currently used by the Router. Multiple DNS IP
                      settings are common. Usually, the first available DNS
                      Server is used.
                    </li>
                    <li>
                      <strong>Name (SSID) </strong> - The SSID of the Router.{" "}
                    </li>
                    <li>
                      <strong>Channel</strong> - The current wireless channel in
                      use.{" "}
                    </li>
                    <li>
                      <strong>Mode</strong> - The current wireless mode which
                      the Router works on.{" "}
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default StatusPage;