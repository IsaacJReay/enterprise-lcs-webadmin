import React, { useState, useEffect } from "react";
import { Layout, Col, Row, Select, Form, Space } from "antd";
import WANStatic from "./wan-static";
import WANDynamic from "./wan-dynamic";
import { IoIosHelpCircle } from "react-icons/io";
import axios from "axios";

const { Content } = Layout;
const { Option } = Select;

const WANSetting = () => {
  const [values, setValues] = useState();
  const [loading, setLoading] = useState(false);
  const [wan, setWan] = useState({});

  // ============auth =============
  const getToken = localStorage.getItem("token");
  const baseUrl = process.env.REACT_APP_API_URL;
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const handleChange = (value) => {
    setValues(value);
  };

  async function fetchData() {
    await axios({
      method: "GET",
      url: `${baseUrl}/settings/wirednetwork/status`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setLoading(true);
        setWan(res.data.wired_network_param);
        setValues(res.data.dhcp);
        setLoading(false);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
    handleChange();
  }, []);

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>WAN SETTING</h1>
                </div>
                <div className="desc-container-banner">
                  <Form>
                    <Form.Item label="WAN Connection type">
                      <Select
                        onChange={handleChange}
                        defaultValue={values === true ? "Dynamic" : "Static"}
                        size="large"
                        className="select-option"
                      >
                        <Option value={true}>Dynamic</Option>
                        <Option value={false}>Static</Option>
                      </Select>
                    </Form.Item>
                  </Form>
                </div>

                {/* ----------form------ */}
                <div className="desc-container-banner2">
                  <center>
                    {values === true ? (
                      <WANDynamic wan={wan} fetchData={fetchData} />
                    ) : (
                      <WANStatic wan={wan} fetchData={fetchData} />
                    )}
                  </center>
                </div>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <div>
                  <p>
                    <strong>WAN Connection Type:</strong>
                  </p>
                  <p>
                    If your ISP is running a DHCP server, select the Dynamic IP
                    option.
                  </p>
                  <p>
                    If your ISP provides a static or fixed IP Address, Subnet
                    Mask, Gateway and DNS setting, select the Static IP option.
                  </p>
                  <p>
                    If you don't know how to choose the appropriate connection
                    type, click the Detect button to allow the Router to
                    automatically search your Internet connection for servers
                    and protocols. The connection type will be reported when an
                    active Internet service is successfully detected by the
                    Router. This report is for your reference only. To make sure
                    the connection type your ISP provides, please refer to the
                    ISP. The various types of Internet connections that the
                    Router can detect are as follows:
                  </p>
                  <ul>
                    <li>
                      <strong>Dynamic IP</strong> - Connections which use
                      dynamic IP address assignment.
                    </li>
                    <li>
                      <strong>Static IP </strong> - Connections which use static
                      IP address assignment.
                    </li>
                  </ul>
                  <p>
                    <strong>IP Address</strong> - The IP address assigned by
                    your ISP dynamically.
                  </p>
                  <p>
                    <strong>Subnet Mask</strong> - The subnet mask assigned by
                    your ISP dynamically.{" "}
                  </p>
                  <p>
                    <strong>Default Gateway</strong> - The default gateway
                    assigned dynamically by your ISP.{" "}
                  </p>
                  <p>
                    <strong>DNS</strong> - Enter the DNS IP address in
                    dotted-decimal notation provided by your ISP.
                  </p>
                  <p>
                    <strong>Note: </strong> If you get Address not found error
                    when you access a Web site, it is likely that your DNS
                    servers are set up improperly. You should contact your ISP
                    to get DNS server addresses.
                  </p>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default WANSetting;
