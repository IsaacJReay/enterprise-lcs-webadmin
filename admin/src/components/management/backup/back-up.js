import React, { useState } from "react";
import { Form, Input, Button, message } from "antd";
import axios from "axios";
import fileDownload from "js-file-download";

const Backup = () => {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  const handleExport = (data) => {
    const inputData = {
      filename: data.filename,
      password: data.password,
    };

    axios
      .post("http://10.42.0.188:8080/private/api/settings/export", inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
          responseType: "blob",
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Failed") {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        } else {
          setLoading(true);
          fileDownload(res.data, data.filename + ".tar.zst");
          message.success("Successful!");
          form.resetFields();
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  return (
    <React.Fragment>
      <div className="container">
        <div className="container-header">
          <h1>Backup Config</h1>
        </div>
        <hr />
        <div className="backup-container">
          <Form {...layout} onFinish={handleExport} form={form}>
            <Form.Item
              label="Name"
              name="filename"
              rules={[
                {
                  required: true,
                  message: "Name is required!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
            </Form.Item>

            <Form.Item
              label="Password"
              name="password"
              rules={[
                {
                  required: true,
                  message: "Input Password!",
                },
              ]}
            >
              <Input.Password className="label-info" size="large" />
            </Form.Item>

            <Form.Item>
              <Button
                type="primary"
                htmlType="submit"
                className="button-apply"
                size="large"
              >
                Export
              </Button>
            </Form.Item>
          </Form>
        </div>
      </div>
    </React.Fragment>
  );
};

export default Backup;
