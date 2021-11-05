import React, { useState } from "react";
import { Button, Form, Input, message } from "antd";
import axios from "axios";

const CreateDomain = () => {
  //   // -----state ---------
  const [loading, setLoading] = useState(false);

  //   // ------- token ----------
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = (data) => {
    const inputData = {
      domain_name: data.domain_name,
    };

    axios
      .post(
        "http://10.42.0.188:8080/private/api/settings/dns/domain_name/creation",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  return (
    <React.Fragment>
      <Form layout="inline" onFinish={handleApply}>
        <Form.Item
          label="Domain name"
          name="domain_name"
          rules={[{ required: true, message: "Input domain name!" }]}
        >
          <Input
            placeholder="text here ..."
            size="large"
            className="input-info"
          />
        </Form.Item>
        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            className="button-update"
            size="large"
          >
            Create
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default CreateDomain;
