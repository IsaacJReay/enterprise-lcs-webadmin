import React, { useState } from "react";
import { Form, Select, Button, Row, Col, message, Modal } from "antd";
import axios from "axios";
import { ExclamationCircleOutlined } from "@ant-design/icons";

const { Option } = Select;
const moment = require("moment-timezone");

const Automaticaly = ({ pick, items }) => {
  const moments = moment.tz.names();
  const defalutTime = moment.tz.guess();
  const [, setLoading] = useState(false);

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // ------- apply button ---------

  const handleApply = () => {
    const inputData = {
      timezone: defalutTime,
    };

    axios
      .post(`${baseUrl}/settings/time/timezone`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          showPromiseConfirm();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  function showPromiseConfirm() {
    Modal.confirm({
      title: "Are you sure?",
      icon: <ExclamationCircleOutlined />,
      content: "When clicked the OK button, this dialog will be logout!",
      onOk() {
        return window.location.replace("/logout");
      },
      onCancel() {},
    });
  }

  return (
    <React.Fragment>
      <Form onFinish={handleApply}>
        <Form.Item>
          {pick === true ? (
            <div>
              <Row gutter={[12, 12]}>
                <Col span={14}>
                  <Form.Item label="Zone" name="zone">
                    <Select
                      defaultValue={defalutTime}
                      size="large"
                      className="select-Option"
                      showSearch
                      optionFilterProp="children"
                    >
                      {moments.map((res) => {
                        return <Option value={res}>{res}</Option>;
                      })}
                    </Select>
                  </Form.Item>
                </Col>
                <Col span={10}>
                  <Form.Item>
                    <Button
                      size="large"
                      type="primary"
                      htmlType="submit"
                      className="button-update"
                    >
                      Update
                    </Button>
                  </Form.Item>
                </Col>
              </Row>
            </div>
          ) : (
            <Row gutter={[12, 12]}>
              <Col span={14}>
                <Form.Item label="Zone">
                  <Select
                    defaultValue={defalutTime}
                    size="large"
                    className="select-Option"
                    showSearch
                    optionFilterProp="children"
                    disabled
                  >
                    {moments.map((res) => {
                      return <Option value={res}>{res}</Option>;
                    })}
                  </Select>
                </Form.Item>
              </Col>
              <Col span={10}>
                <Form.Item>
                  <Button
                    size="large"
                    type="primary"
                    htmlType="submit"
                    className="button-update"
                    disabled
                  >
                    Update
                  </Button>
                </Form.Item>
              </Col>
            </Row>
          )}
        </Form.Item>
        <Form.Item label="Time">
          <time className="date-time">{items.time}</time>
        </Form.Item>
        <Form.Item label="Date">
          <time className="date-time">{items.date}</time>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default Automaticaly;
