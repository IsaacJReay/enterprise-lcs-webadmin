import React from "react";
import { Row, Col, Layout } from "antd";
import Backup from "./back-up";
import Restore from "./restore";

const { Content } = Layout;

const BackupSetting = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="contaner-update-class">
              <div>
                <Restore />
              </div>
              <div>
                <Backup />
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

export default BackupSetting;
