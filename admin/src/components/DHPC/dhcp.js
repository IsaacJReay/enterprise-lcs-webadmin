import React from "react";
import { Layout, Col, Row, Button, Input, Form } from "antd";

const { Content } = Layout;

const DHCPSetting = () => {
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
                  <Form.Item label="Default Getway">
                    <Input
                      size="large"
                      placeholder="0.0.0.0"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item label="DHCP Range ">
                    <Row gutter={[12, 0]}>
                      <Col>
                        <Input size="large" placeholder="0.0.0.0" />
                      </Col>
                      <Col>
                        <Input size="large" placeholder="0.0.0.0" />
                      </Col>
                    </Row>
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
                  <Form.Item label="DNS ">
                    <Input
                      size="large"
                      placeholder="0.0.0.0"
                      className="label-info"
                    />
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
    </React.Fragment>
  );
};

export default DHCPSetting;
