import React, { useState } from "react";
import { Layout, Col, Row, Select, Form } from "antd";
import WANStatic from "./wan-static";
import WANDynamic from "./wan-dynamic";

const { Content } = Layout;
const { Option } = Select;

const WANSetting = () => {
  const [values, setValues] = useState("Dynamic");

  const handleChange = (value) => {
    return setValues(value);
  };
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>WAN Setting</h1>
                </div>
                <hr />
                <div className="desc-container-banner">
                  <Form {...layout}>
                    <Form.Item label="WAN Connection type">
                      <Row gutter={[32, 32]}>
                        <Col>
                          <Select
                            onChange={handleChange}
                            defaultValue="Dynamic"
                            size="large"
                            className="select-option"
                          >
                            <Option value="Dynamic">Dynamic</Option>
                            <Option value="Static">Static</Option>
                          </Select>
                        </Col>
                      </Row>
                    </Form.Item>
                  </Form>
                </div>

                {/* ----------form------ */}
                <div className="desc-container-banner">
                  {values === "Dynamic" ? <WANDynamic /> : <WANStatic />}
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
                  <h2>WAN Help</h2>
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
