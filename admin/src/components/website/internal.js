import React from "react";
import { Layout, Col, Row, Table, Tag } from "antd";
import data from "./api/data.json";

const { Content } = Layout;

const InternalWebsite = () => {
  const columns = [
    {
      title: "No",
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
      title: "Actions",
      dataIndex: "id",
      width: "20%",
      render: () => {
        return (
          <React.Fragment>
            <Tag color="processing" style={{ cursor: "pointer" }}>
              Update
            </Tag>
            <Tag color="error" style={{ cursor: "pointer" }}>
              Disable
            </Tag>
          </React.Fragment>
        );
      },
    },
  ];

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
          <div className="card">
            <div className="container">
              <div className="container-header">
                <h1>Internal Website</h1>
              </div>
              <hr />
              <Table
                columns={columns}
                dataSource={data}
                pagination={false}
                scroll={{ y: 450 }}
              />
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

export default InternalWebsite;
