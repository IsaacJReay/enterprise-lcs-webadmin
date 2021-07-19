import React from "react";
import { Layout, Col, Row, Collapse, Progress } from "antd";
import { CaretRightOutlined } from "@ant-design/icons";
import myImage from "../../assets/images/storages.png";
import { Link } from "react-router-dom";

const { Content } = Layout;
const { Panel } = Collapse;

const Storages = () => {
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
              <Collapse
                bordered={false}
                defaultActiveKey={["1"]}
                expandIcon={({ isActive }) => (
                  <CaretRightOutlined rotate={isActive ? 90 : 0} />
                )}
              >
                <Panel header="All devices (2)" key="1">
                  <div className="storages-container">
                    <div className="item-storages">
                      <Link to="storages/local">
                        <Row gutter={24}>
                          <Col span={3}>
                            <img
                              src={myImage}
                              className="storages-icon"
                              alt="storages picture"
                            />
                          </Col>
                          <Col span={21}>
                            <p>Local Content Storage</p>
                            <Progress
                              percent={70}
                              showInfo={false}
                              strokeWidth={25}
                              status="active"
                              strokeColor={{
                                "0%": "#65DDFF",
                                "100%": "#E2F516",
                              }}
                            />
                            <p>3T free of 4T</p>
                          </Col>
                        </Row>
                      </Link>
                    </div>
                    <div className="item-storages">
                      <Row gutter={24}>
                        <Col span={3}>
                          <img
                            src={myImage}
                            className="storages-icon"
                            alt="storages picture"
                          />
                        </Col>
                        <Col span={21}>
                          <p>Removeable Device</p>
                          <Progress
                            percent={30}
                            showInfo={false}
                            strokeWidth={25}
                            status="active"
                            strokeColor={{
                              "0%": "#65DDFF",
                              "100%": "#E2F516",
                            }}
                          />
                          <p>3T free of 4T</p>
                        </Col>
                      </Row>
                    </div>
                  </div>
                </Panel>
              </Collapse>
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

export default Storages;
