import React from "react";
import { Layout, Col, Row, Table } from "antd";
import data from "./api/data.json";
import { SyncOutlined } from "@ant-design/icons";

const { Content } = Layout;

const CustomWebsite = () => {
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
  ];

  function handleRefresh() {
    window.location.reload();
  }

  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>Custom Hosting</h1>
                <SyncOutlined
                  className="refresh-page"
                  onClick={handleRefresh}
                />
              </div>
              <hr />
              <Table
                columns={columns}
                dataSource={data}
                pagination={false}
                scroll={{ y: 450 }}
              />
            </div>
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

export default CustomWebsite;
