import React, { useState } from "react";
import { Form, Input, Button, message } from "antd";
import axios from "axios";
import fileDownload from "js-file-download";

const Backup = () => {
  const [, setLoading] = useState(false);
  const [form] = Form.useForm();

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
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
      .post(`${baseUrl}/settings/export`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
          responseType: "blob",
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          fileDownload(res.data, data.filename + ".tar.zst");
          message.success("Successful!");
          form.resetFields();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  return (
    <React.Fragment>
      <Form
        {...layout}
        onFinish={handleExport}
        form={form}
        className="backup-container"
      >
        <Form.Item
          label="File Name"
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
            className="button-export"
            size="large"
          >
            EXPORT
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default Backup;
