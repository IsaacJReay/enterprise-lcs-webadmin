import React from "react";
import { Row, Col, Layout } from "antd";
import Backup from "./back-up";
import Restore from "./restore";

const { Content } = Layout;

const BackupSetting = () => {
  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
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
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
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
