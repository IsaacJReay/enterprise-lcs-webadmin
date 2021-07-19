import React from "react";
import { Row, Col, Layout, Button } from "antd";
import { FiArrowLeft } from "react-icons/fi";
import { CaretRightOutlined } from "@ant-design/icons";
import { Link } from "react-router-dom";
import LocalTable from "./local-table";

const { Content } = Layout;

const LocalStorage = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>File Storages</h1>
              </div>
              <hr />
              <div className="header-storages">
                <Button type="primary" shape="circle" size="large">
                  <Link to="/storages">
                    <FiArrowLeft className="back-btn" />
                  </Link>
                </Button>
                <div className="header-storage-2">
                  <CaretRightOutlined />
                  Local Storage
                </div>
              </div>
              <div className="btn-options-storages">
                <Button type="primary" size="large">
                  Copy
                </Button>
                <Button type="primary" size="large">
                  Past
                </Button>
                <Button type="primary" size="large">
                  Delete
                </Button>
              </div>
              <hr />
              <div className="local-data-container">
                <LocalTable />
              </div>
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

export default LocalStorage;
