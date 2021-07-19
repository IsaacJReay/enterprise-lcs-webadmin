import React from "react";
import {
  Layout,
  Col,
  Row,
  Select,
  Button,
  Input,
  Form,
  Checkbox,
  Radio,
} from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";

const { Content } = Layout;
const { Option } = Select;

const WirelessSetting = () => {
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
      {/* <Content>
            <Row gutter={[32, 32]}>
              <Col span={16}>
                <Form>
                  <div className="container">
                    <div className="container-header">
                      <h1>Wireless Setting</h1>
                    </div>
                    <hr />
                    <div className="desc-container-banner">
                      <React.Fragment>
                        <Row gutter={[64, 0]}>
                          <Col span={8}>
                            <div className="desc-details-left">
                              <Row gutter={[0, 12]}>
                                <Col span={24}>
                                  <p>Network Name SSID : </p>
                                </Col>
                                <Col span={24}>
                                  <p>Security : </p>
                                </Col>
                                <Col span={24}>
                                  <p>Version: </p>
                                </Col>
                                <Col span={24}>
                                  <p>Password : </p>
                                </Col>
                                <Col span={24}>
                                  <p>Mood : </p>
                                </Col>
                                <Col span={24}>
                                  <p>Channel Width : </p>
                                </Col>
                                <Col span={24}>
                                  <p>Channel : </p>
                                </Col>
                              </Row>
                            </div>
                          </Col>
                          <Col span={16}>
                            <div className="desc-details-right">
                              <Row gutter={[0, 18]}>
                                <Col span={24}>
                                  <Row gutter={[12, 0]}>
                                    <Col span={12}>
                                      <Input
                                        size="large"
                                        placeholder="text here ..."
                                        className="label-info1"
                                      />
                                    </Col>
                                    <Col span={12}>
                                      <Checkbox> Hide SSID</Checkbox>
                                    </Col>
                                  </Row>
                                </Col>
                                <Col span={24}>
                                  <Select
                                    defaultValue="WP2/WPA2-Personal (Recommended)"
                                    size="large"
                                    className="select-option-wireless"
                                  >
                                    <Option value="WP2/WPA2-Personal (Recommended)">
                                      WP2/WPA2-Personal (Recommended)
                                    </Option>
                                    <Option value="WP2/WPA2-Personal">
                                      WP2/WPA2-Personal
                                    </Option>
                                  </Select>
                                </Col>
                                <Col span={24}>
                                  <Radio.Group>
                                    <Radio value="WPA2-SPK">WPA2-SPK</Radio>
                                    <Radio value="WPA-SPK">WPA-SPK</Radio>
                                  </Radio.Group>
                                </Col>

                                <Col span={24}>
                                  <Input.Password
                                    size="large"
                                    placeholder="password"
                                    className="label-info"
                                  />
                                </Col>
                                <Col span={24}>
                                  <Input
                                    size="large"
                                    placeholder="0.0.0.0"
                                    className="label-info"
                                  />
                                </Col>
                                <Col span={24}>
                                  <Input
                                    size="large"
                                    placeholder="0.0.0.0"
                                    className="label-info"
                                  />
                                </Col>
                                <Col span={24}>
                                  <Select
                                    defaultValue=" Asia/Phnom Penh"
                                    size="large"
                                    className="select-option"
                                  >
                                    <Option value="Asia/Phnom Penh">
                                      Asia/Phnom Penh
                                    </Option>
                                    <Option value=" Asia/Bangkok">
                                      Asia/Bangkok
                                    </Option>
                                  </Select>
                                </Col>
                              </Row>
                            </div>
                          </Col>
                        </Row>
                      </React.Fragment>
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
          </Content> */}
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <Form {...layout}>
              <div className="container">
                <div className="container-header">
                  <h1>Wireless Setting</h1>
                </div>
                <hr />

                <div className="desc-container-banner">
                  <Form.Item label="Network Name SSID">
                    <Row gutter={[12, 0]}>
                      <Col>
                        <Input size="large" placeholder="Text here ..." />
                      </Col>
                      <Col>
                        <Checkbox>Hide SSID</Checkbox>
                      </Col>
                    </Row>
                  </Form.Item>
                  <Form.Item label="Security ">
                    <Select
                      defaultValue="WP2/WPA2-Personal (Recommended)"
                      size="large"
                      className="select-option-wireless"
                    >
                      <Option value="WP2/WPA2-Personal (Recommended)">
                        WP2/WPA2-Personal (Recommended)
                      </Option>
                      <Option value="WP2/WPA2-Personal">
                        WP2/WPA2-Personal
                      </Option>
                    </Select>
                  </Form.Item>
                  <Form.Item label="Version ">
                    <Radio.Group>
                      <Radio value="WPA2-SPK">WPA2-SPK</Radio>
                      <Radio value="WPA-SPK">WPA-SPK</Radio>
                    </Radio.Group>
                  </Form.Item>
                  <Form.Item label="Password ">
                    <Input.Password
                      size="large"
                      placeholder="password"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item label="Mood">
                    <Select
                      defaultValue=" 802.11g"
                      size="large"
                      className="select-option-wireless"
                    >
                      <Option value="  802.11g">802.11g</Option>
                      <Option value="  902.11g">802.11g</Option>
                    </Select>
                  </Form.Item>
                  <Form.Item label="Channel Width  ">
                    <Select
                      defaultValue="WP2/WPA2-Personal (Recommended)"
                      size="large"
                      className="select-option-wireless"
                    >
                      <Option value="WP2/WPA2-Personal (Recommended)">
                        WP2/WPA2-Personal (Recommended)
                      </Option>
                      <Option value="WP2/WPA2-Personal">
                        WP2/WPA2-Personal
                      </Option>
                    </Select>
                  </Form.Item>
                  <Form.Item label="Channel ">
                    <Row gutter={[0, 12]}>
                      <Col span={24}>
                        <Select
                          defaultValue=" 1"
                          size="large"
                          className="select-option"
                        >
                          <Option value="1">1</Option>
                          <Option value="2">2</Option>
                          <Option value="3">3</Option>
                          <Option value="4">4</Option>
                        </Select>
                      </Col>
                      <Col span={24}>
                        <Radio.Group>
                          <Radio value="QOS">QOS</Radio>
                          <Radio value="802.11N">802.11N</Radio>
                        </Radio.Group>
                      </Col>
                    </Row>
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

export default WirelessSetting;
