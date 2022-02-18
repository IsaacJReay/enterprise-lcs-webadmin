import React, { useState } from "react";
import { Layout, Col, Row, Button, Input, Form, Space } from "antd";
import Logo from "./../../../assets/images/icons/koompi-black.png";
import { DownloadOutlined, SearchOutlined } from "@ant-design/icons";
import AdvanceSearch from "./advance-search";

const { Content } = Layout;

const DownloadWebsite = () => {
  // ---------state --------------
  const [search, setSearch] = useState(false);

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
          <div className="card">
            <div className="container">
              <div className="container-header">
                <h1>Download Website</h1>
              </div>
              <hr />
              <center>
                <div>
                  <img src={Logo} className="website-logo" alt="logo" />
                </div>
                <div className="website-form">
                  <Form layout="vertical">
                    <Form.Item
                      name="website"
                      label="Website"
                      rules={[
                        {
                          required: true,
                          type: "url",
                          warningOnly: true,
                        },
                      ]}
                    >
                      <Input size="large" className="input-info" />
                    </Form.Item>
                    <Form.Item
                      name="destination"
                      label="Destination"
                      rules={[
                        {
                          required: true,
                          type: "url",
                          warningOnly: true,
                        },
                      ]}
                    >
                      <Input size="large" />
                    </Form.Item>

                    <Space>
                      <Form.Item>
                        <Button
                          type="primary"
                          htmlType="submit"
                          size="large"
                          className="button-update"
                        >
                          <DownloadOutlined />
                          DownLoad
                        </Button>
                      </Form.Item>
                      <Form.Item>
                        <Button
                          // htmlType="button"
                          size="large"
                          className="advance-search-button"
                          onClick={() => setSearch(!search)}
                        >
                          <SearchOutlined />
                          Advance Search
                        </Button>
                      </Form.Item>
                    </Space>
                  </Form>
                </div>
              </center>
              {search && (
                <div className="container-advance-search">
                  <AdvanceSearch />
                </div>
              )}
            </div>
            </div>
          </Col>
          <Col span={8}>
          <div className="card">
            <div className="container">
              <div className="container-header">
                <h1>Desciptions</h1>
              </div>
            </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default DownloadWebsite;
