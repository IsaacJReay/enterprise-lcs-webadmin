import React from "react";
import { Layout, Col, Row, Select, Button, Input, Form } from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";

const { Content } = Layout;
const { Option } = Select;

const WLANSetting = () => {
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
                      <h1>WLAN Setting</h1>
                    </div>
                    <hr />

                    <div className="desc-container-banner">
                      <Form.Item label="Router Address">
                        <Input
                          size="large"
                          placeholder="0.0.0.0"
                          className="label-info"
                        />
                      </Form.Item>
                      <Form.Item label="Netmask">
                        <Input
                          size="large"
                          placeholder="0.0.0.0"
                          className="label-info"
                        />
                      </Form.Item>
                      <Form.Item label="Range ">
                        <Row gutter={[12, 0]}>
                          <Col>
                            <Input size="large" placeholder="0.0.0.0" />
                          </Col>
                          <Col>
                            <Input size="large" placeholder="0.0.0.0" />
                          </Col>
                        </Row>
                      </Form.Item>
                      <Form.Item label="DNS ">
                        <Input
                          size="large"
                          placeholder="0.0.0.0"
                          className="label-info"
                        />
                      </Form.Item>
                      <Form.Item label="Default Lease time">
                        <Input
                          size="large"
                          placeholder="0.0.0.0"
                          className="label-info"
                        />
                      </Form.Item>
                      <Form.Item label="Max Lease time  ">
                        <Input
                          size="large"
                          placeholder="0.0.0.0"
                          className="label-info"
                        />
                      </Form.Item>
                      <Form.Item label="Timezone ">
                        <Select
                          defaultValue=" Asia/Phnom Penh"
                          size="large"
                          className="select-option"
                        >
                          <Option value="Asia/Phnom Penh">
                            Asia/Phnom Penh
                          </Option>
                          <Option value=" Asia/Bangkok">Asia/Bangkok</Option>
                        </Select>
                      </Form.Item>
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

export default WLANSetting;
