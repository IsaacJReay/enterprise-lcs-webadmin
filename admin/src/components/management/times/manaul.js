import React, { useState } from "react";
import { Form, Button, DatePicker, TimePicker } from "antd";

const CustomeTime = ({ pick }) => {
  const [value, setValue] = useState(null);

  const onChange = (time) => {
    setValue(time);
  };

  return (
    <React.Fragment>
      <Form>
        {pick === "manual" ? (
          <React.Fragment>
            <Form.Item
              label="Time"
              name="time"
              rules={[
                {
                  required: true,
                  message: "Time is required!",
                },
              ]}
            >
              <TimePicker
                value={value}
                onChange={onChange}
                size="large"
                className="time-pickup"
                use12Hours
                format="h:mm:ss A"
              />
            </Form.Item>
            <Form.Item
              label="Date"
              name="date"
              rules={[
                {
                  required: true,
                  message: "Date is required!",
                },
              ]}
            >
              <DatePicker className="time-pickup" size="large" />
            </Form.Item>
            <Form.Item>
              <Button
                size="large"
                className="button-update"
                type="primary"
                htmlType="button"
              >
                Apply
              </Button>
            </Form.Item>
          </React.Fragment>
        ) : (
          <React.Fragment>
            <Form.Item
              label="Time"
              name="time"
              rules={[
                {
                  required: true,
                  message: "Time is required!",
                },
              ]}
            >
              <TimePicker
                value={value}
                onChange={onChange}
                disabled
                className="time-pickup"
                size="large"
              />
            </Form.Item>
            <Form.Item
              label="Date"
              name="date"
              rules={[
                {
                  required: true,
                  message: "Date is required!",
                },
              ]}
            >
              <DatePicker className="time-pickup" size="large" disabled />
            </Form.Item>
            <Form.Item>
              <Button
                size="large"
                className="button-update"
                type="primary"
                htmlType="button"
                disabled
              >
                Apply
              </Button>
            </Form.Item>
          </React.Fragment>
        )}
      </Form>
    </React.Fragment>
  );
};

export default CustomeTime;
