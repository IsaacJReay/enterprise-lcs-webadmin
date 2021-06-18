import React from "react";
import {
  Layout,
  Col,
  Row,
  Button,
  Input,
  Form,
  Table,
  Checkbox,
  Tag,
} from "antd";
import NavBar from "../layouts/navbar";
import SideNavBar from "../layouts/side-navbar";
import { Link } from "react-router-dom";

const { Content } = Layout;

const DNSSetting = () => {
  const data = [
    {
      id: "1",
      name: "sala.koompi.com",
      status: true,
    },
    {
      id: "2",
      name: "koompi.com",
      status: false,
    },
    {
      id: "3",
      name: "koompi.org",
      status: false,
    },
  ];
  const columns = [
    {
      title: "N0",
      dataIndex: "id",
      key: "id",
      width: "10%",
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Status",
      dataIndex: " status",
      key: "status",
      render: () => {
        return <Checkbox />;
      },
    },
    {
      title: "Actions",
      dataIndex: "address",
      render: () => {
        return (
          <React.Fragment>
            <Link to="/dns-management">
              <Tag color="processing">Manage</Tag>
            </Link>
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
                        <Row gutter={[24, 0]}>
                          <Col span={12}>
                            <Input size="large" placeholder="Text here ..." />
                          </Col>
                          <Col span={12}>
                            <Button
                              type="primary"
                              htmlType="button"
                              size="large"
                            >
                              Create
                            </Button>
                          </Col>
                        </Row>
                      </Form.Item>
                    </div>
                    <div className="dns-desc-container">
                      <Table
                        columns={columns}
                        dataSource={data}
                        pagination={false}
                        scroll={{ y: 350 }}
                      />
                    </div>
                    <Form.Item>
                      <Button
                        type="primary"
                        htmlType="button"
                        className="button-apply2"
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

export default DNSSetting;
