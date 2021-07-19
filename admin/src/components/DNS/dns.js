import React, { useState } from "react";
import { Layout, Col, Row, Button, Form, Table, Checkbox, Tag } from "antd";

import { Link } from "react-router-dom";
import { FiPlus } from "react-icons/fi";
import CreateDomain from "./create-domain";

const { Content } = Layout;

const DNSSetting = () => {
  const [visible, setVisible] = useState(false);

  const handleCancel = () => {
    setVisible(false);
  };
  const handleOk = () => {
    setVisible(false);
  };

  const createDomain = () => {
    setVisible(true);
  };

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
        return <Checkbox disabled />;
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
            <Tag color="warning">Edit</Tag>
            <Tag color="error">Delete</Tag>
          </React.Fragment>
        );
      },
    },
  ];
  return (
    <React.Fragment>
      <Content>
        <CreateDomain
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
                  <Form.Item>
                    <Button type="primary" onClick={createDomain}>
                      <FiPlus className="add-button" />
                      Create Domain
                    </Button>
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

export default DNSSetting;
