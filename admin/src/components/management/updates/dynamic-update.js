import React from "react";
import { Row, Col, Button } from "antd";
import { SyncOutlined, UpCircleOutlined } from "@ant-design/icons";

const DynamicUpdate = () => {
  return (
    <React.Fragment>
      <div className="card">
        <div className="container">
          <div className="container-header">
            <h1>Automatic Update</h1>
          </div>
          <hr />
          <div className="sub-header">
            <div className="icons-update">
              <SyncOutlined className="updates" />
              Check For Update
            </div>
            <div className="icons-update">
              <UpCircleOutlined className="updates" />
              Update
            </div>
          </div>
          <div className="container-update">
            <Row gutter={[12, 12]}>
              <Col span={21}>
                <div>
                  <h3>content-server_v0.2.202100</h3>
                  <p>Size: 30MB</p>
                </div>
              </Col>
              <Col span={3}>
                <Button type="primary" className="button-update">
                  Update
                </Button>
              </Col>
            </Row>
          </div>
        </div>
      </div>
    </React.Fragment>
  );
};

export default DynamicUpdate;
