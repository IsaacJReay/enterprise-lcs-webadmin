import React from "react";
import { Layout, Col, Row, Collapse } from "antd";
import { SyncOutlined } from "@ant-design/icons";
import DirectoryHosting from "./directory-hosting";
import DownloadHosting from "./download-hosting";

const { Content } = Layout;
const { Panel } = Collapse;

const CustomWebsite = () => {
  function handleRefresh() {
    window.location.reload();
  }

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
          <div className="card">
            <div className="container">
              <div className="container-header">
                <h1>Custom Hosting</h1>
                <SyncOutlined
                  className="refresh-page"
                  onClick={handleRefresh}
                />
              </div>
              <hr />
              <Collapse bordered={false} defaultActiveKey={["1"]}>
                <Panel
                  className="storage-header"
                  key="1"
                  header={<span>Directory Hosting (3)</span>}
                >
                  <DirectoryHosting />
                </Panel>
                <Panel
                  className="storage-header"
                  key="2"
                  header={<span>Download Hosting (3)</span>}
                >
                  <DownloadHosting />
                </Panel>
              </Collapse>
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

export default CustomWebsite;
