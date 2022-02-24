import React from "react";
import { Row, Col, Layout } from "antd";
import DynamicUpdate from "./dynamic-update";
import ManaullyUpdate from "./manually-update";

const { Content } = Layout;

const SystemUpdates = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="contaner-update-class">
              <div>
                <DynamicUpdate />
              </div>
              <div>
                <ManaullyUpdate />
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
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

export default SystemUpdates;
