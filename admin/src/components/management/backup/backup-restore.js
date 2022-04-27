import React from "react";
import { Row, Col, Layout, Tabs, Space } from "antd";
import Backup from "./back-up";
import Restore from "./restore";
import { ThunderboltOutlined, AimOutlined } from "@ant-design/icons";
import { IoIosHelpCircle } from "react-icons/io";

const { TabPane } = Tabs;

const { Content } = Layout;

const BackupSetting = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card2">
              <div className="container">
                <div className="contaner-update-class">
                  <div className="container-header">
                    <h1> CONFIGURATIONS</h1>
                  </div>
                  <Tabs defaultActiveKey="1">
                    <TabPane
                      tab={
                        <span>
                          <ThunderboltOutlined />
                          RESOTRE
                        </span>
                      }
                      key="1"
                    >
                      <Restore />
                    </TabPane>
                    <TabPane
                      tab={
                        <span>
                          <AimOutlined />
                          BACKUP
                        </span>
                      }
                      key="2"
                    >
                      <Backup />
                    </TabPane>
                  </Tabs>
                </div>
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
                <div>
                  <h1>Backup & Restore Help</h1>
                  <p>
                    Click the Backup button to save all configuration settings
                    to your local computer as a file.
                  </p>
                  <p>
                    To restore this device's configuration, follow these
                    instructions:
                  </p>
                  <ul>
                    <li>
                      Click the Browse button to find the configuration file
                      which you want to restore.
                    </li>
                    <li>
                      Click the Restore button to update the configuration with
                      the file whose path is the one you have input or selected
                      in the blank.
                    </li>
                  </ul>
                  <p>
                    <strong>Note: </strong>Note: The current configuration will
                    be covered with the uploading configuration file. Wrong
                    process will lead this device unmanaged. The restoring
                    process lasts for 20 seconds and this device will restart
                    automatically then. Keep the power of this device on during
                    the process, in case of any damage.
                  </p>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default BackupSetting;
