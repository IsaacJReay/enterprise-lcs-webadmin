import React from "react";
import { Row, Col, Layout, Tabs, Button, Space } from "antd";
import DynamicUpdate from "./dynamic-update";
import ManaullyUpdate from "./manually-update";
import { ThunderboltOutlined, AimOutlined } from "@ant-design/icons";
import { IoIosHelpCircle } from "react-icons/io";

const { TabPane } = Tabs;

const { Content } = Layout;

const SystemUpdates = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <h1> SYSTEM UPDATES</h1>
                </div>
                <Tabs defaultActiveKey="1">
                  <TabPane
                    tab={
                      <span>
                        <ThunderboltOutlined />
                        DYNAMICALLY
                      </span>
                    }
                    key="1"
                  >
                    <DynamicUpdate />
                  </TabPane>
                  {/* <TabPane
                    tab={
                      <span>
                        <AimOutlined />
                        MANUALLY
                      </span>
                    }
                    key="2"
                  >
                    <ManaullyUpdate />
                  </TabPane> */}
                </Tabs>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
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
