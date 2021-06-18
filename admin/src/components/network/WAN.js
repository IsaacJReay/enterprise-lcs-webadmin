import React, { useState } from "react";
import { Layout, Col, Row, Select, Button, Input, Form } from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";

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
      <Layout style={{ minHeight: "100vh" }}>
        <NavBar />
        <Layout>
          <SideNavBar />
          <Content>
            <Row gutter={[32, 32]}>
              <Col span={16}>
                <Form {...layout}>
                  <div className="container">
                    <div className="container-header">
                      <h1>WAN Setting</h1>
                    </div>
                    <hr />
                    <div className="desc-container-banner">
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
                          <Col>
                            <Form.Item>
                              {values === "Dynamic" && (
                                <Button
                                  type="primary"
                                  size="large"
                                  className="detect-button"
                                >
                                  Detect
                                </Button>
                              )}
                              {values === "Static" && (
                                <Button
                                  disabled
                                  type="primary"
                                  size="large"
                                  className="detect-button"
                                >
                                  Detect
                                </Button>
                              )}
                            </Form.Item>
                          </Col>
                        </Row>
                      </Form.Item>
                    </div>
                    <div className="desc-container-banner">
                      {values === "Dynamic" && (
                        <React.Fragment>
                          <Form.Item label="MAC Address">
                            <Input
                              disabled
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="IP Address">
                            <Input
                              disabled
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="Subnet Mask">
                            <Input
                              disabled
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="Default Getway">
                            <Input
                              disabled
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                        </React.Fragment>
                      )}
                      {values === "Static" && (
                        <React.Fragment>
                          <Form.Item label="MAC Address">
                            <Input
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="IP Address">
                            <Input
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="Subnet Mask">
                            <Input
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                          <Form.Item label="Default Getway">
                            <Input
                              size="large"
                              placeholder="0.0.0.0"
                              className="label-info"
                            />
                          </Form.Item>
                        </React.Fragment>
                      )}
                    </div>
                    <Form.Item>
                      <Button
                        type="primary"
                        htmlType="button"
                        className="button-apply"
                        size="large"
                      >
                        Apply
                      </Button>
                    </Form.Item>
                  </div>
                </Form>
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

export default WANSetting;
