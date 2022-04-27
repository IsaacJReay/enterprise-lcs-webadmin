import React, { useState } from "react";
import {
  Form,
  Button,
  DatePicker,
  TimePicker,
  message,
  ConfigProvider,
  Modal,
} from "antd";
import axios from "axios";
import "moment/locale/zh-cn";
import locale from "antd/lib/locale/en_US";
import moment from "moment";
import { ExclamationCircleOutlined } from "@ant-design/icons";

const CustomeTime = ({ pick }) => {
  const [setLoading] = useState(false);
  let now = new Date();

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // ------- apply button ---------

  const handleApply = (data) => {
    const inputData = {
      time: data.time,
      date: data.date,
    };

    axios
      .post(`${baseUrl}/settings/time/timedate`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
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
        {pick === false ? (
          <React.Fragment>
            <Form.Item label="Time" name="time">
              <TimePicker
                defaultValue={moment(now, "HH:mm:ss")}
                defaultOpen={moment(now, "HH:mm:ss")}
                size="large"
                className="time-pickup"
                use12Hours
                format="h:mm:ss A"
                showNow={true}
              />
            </Form.Item>
            <Form.Item label="Date" name="date">
              <DatePicker
                showToday
                className="time-pickup"
                size="large"
                defaultValue={moment(now, " YYYY/MM/DD")}
                defaultPickerValue={moment(now, " YYYY/MM/DD")}
              />
            </Form.Item>
            <Form.Item>
              <Button
                size="large"
                className="button-update-time2"
                type="primary"
                htmlType="submit"
              >
                SAVE & APPLY
              </Button>
            </Form.Item>
          </React.Fragment>
        ) : (
          <React.Fragment>
            <Form.Item label="Time" name="time">
              <TimePicker
                defaultValue={moment(now, "HH:mm:ss")}
                defaultOpen={moment(now, "HH:mm:ss")}
                size="large"
                className="time-pickup"
                use12Hours
                format="h:mm:ss A"
                disabled
              />
            </Form.Item>
            <Form.Item label="Date" name="date">
              <ConfigProvider locale={locale}>
                <DatePicker
                  defaultValue={moment(now, " YYYY/MM/DD")}
                  defaultPickerValue={moment(now, " YYYY/MM/DD")}
                  showToday
                  className="time-pickup"
                  size="large"
                  disabled
                />
              </ConfigProvider>
            </Form.Item>
            <Form.Item>
              <Button
                size="large"
                className="button-update-time2"
                type="primary"
                htmlType="submit"
                disabled
              >
                SAVE & APPLY
              </Button>
            </Form.Item>
          </React.Fragment>
        )}
      </Form>
    </React.Fragment>
  );
};

export default CustomeTime;
