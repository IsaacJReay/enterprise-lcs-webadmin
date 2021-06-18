import React, { useState } from "react";
import {
  Layout,
  Col,
  Row,
  Button,
  Form,
  Table,
  Select,
  Tag,
  Popover,
} from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";
import dataDns from "./data.json";
import { FiEdit, FiPlus } from "react-icons/fi";
import DNSRename from "./rename";
import { Link } from "react-router-dom";

const { Content } = Layout;
const { Option } = Select;

const DNSManagement = () => {
  const [visible, setVisible] = useState(false);

  const showModalRename = () => {
    setVisible(true);
  };

  const handleCancel = () => {
    setVisible(false);
  };

  const handleOk = () => {
    setVisible(false);
  };

  const columns = [
    {
      title: "N0",
      dataIndex: "id",
      key: "id",
      width: "10%",
    },
    {
      title: "Sub Domain",
      dataIndex: "sub-domain",
      width: "20%",
      key: "sub-domain",
    },
    {
      title: "Address",
      dataIndex: "address",
      width: "20%",
      key: "address",
    },
    {
      title: "Type",
      dataIndex: "types",
      key: "types",
      render: (types) => {
        return (
          <React.Fragment>
            <Select defaultValue="A" size="large" className="select-option">
              <Option value="A">A</Option>
              <Option value="MX 10">MX 10</Option>
              <Option value="CNAME">CNAME</Option>
            </Select>
          </React.Fragment>
        );
      },
    },
    {
      title: "Actions",
      width: "10%",
      render: () => {
        return (
          <React.Fragment>
            <Tag color="error">Delete</Tag>
          </React.Fragment>
        );
      },
    },
  ];
  return (
    <React.Fragment>
      <Layout style={{ minHeight: "100vh" }}>
        <NavBar />
        <Layout>
          <SideNavBar />
          <Content>
            <DNSRename
              visible={visible}
              handleCancel={handleCancel}
              handleOk={handleOk}
            />
            <Row gutter={[32, 32]}>
              <Col span={16}>
                <Form>
                  <div className="container">
                    <div className="container-header">
                      <h1>DNS Setting</h1>
                    </div>
                    <hr />

                    <div className="dns-desc-container">
                      <Form.Item label="Domain Name">
                        <Row gutter={[6, 0]}>
                          <Col>
                            <h2>sala.koompi.com</h2>
                          </Col>
                          <Col>
                            <Popover
                              title={null}
                              content="Rename"
                              placement="topLeft"
                            >
                              <FiEdit onClick={showModalRename} />
                            </Popover>
                          </Col>
                        </Row>
                      </Form.Item>
                    </div>
                    <div className="dns-desc-container">
                      <Form.Item>
                        <Button type="primary">
                          <FiPlus className="add-button" />
                          Add Record
                        </Button>
                      </Form.Item>
                      <Table
                        columns={columns}
                        dataSource={dataDns}
                        pagination={false}
                        scroll={{ y: 350 }}
                      />
                    </div>

                    <Form.Item>
                      <div className="container-buttons">
                        <Row gutter={12}>
                          <Col>
                            <Link to="/dns">
                              <Button
                                type="primary"
                                htmlType="button"
                                className="button-apply2"
                                size="large"
                              >
                                Back
                              </Button>
                            </Link>
                          </Col>
                          <Col>
                            <Button
                              type="primary"
                              htmlType="button"
                              className="button-apply2"
                              size="large"
                            >
                              Apply
                            </Button>
                          </Col>
                        </Row>
                      </div>
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

export default DNSManagement;
